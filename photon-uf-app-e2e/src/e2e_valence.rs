//! Process-wide Valence + Higgs + mem Photon for Playwright.
#![allow(dead_code)]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

use chrono::Utc;
use gauge::manifest_sync::{
    sync_permission_manifests, PermissionDomainInput, PermissionInput, PermissionManifestInput,
};
use gauge::service;
use gauge::super_user::SUPER_USER_GROUP_NAME;
use higgs::actor_policy::external_actor_json_policy;
use higgs::{HiggsConfig, HiggsValenceFactory};
use photon::{configure, subscribe, topic, Actor, JsonIdentityFactory, Photon};
use tokio::sync::OnceCell;
use valence::{
    register_backend_logical_names, router_key, Actor as ValenceActor, DatabaseBackend,
    DatabaseRouter, InMemoryBackend, Model, RegisterBackendLogicalNamesOptions,
    RouterValenceFactory, RouterValenceFactoryConfig, Valence, ValenceFactory, MEM_ENGINE_ID,
    SQLITE_ENGINE_ID,
};

struct E2eState {
    router: Arc<DatabaseRouter>,
    higgs: Arc<HiggsConfig>,
    photon: Arc<Photon>,
    default_backend_key: String,
    fixtures: Mutex<FixtureIds>,
}

/// Stable fixture ids exposed to seed JSON / Playwright.
#[derive(Clone, Debug, Default)]
pub struct FixtureIds {
    pub topic_name: String,
    pub subscription_id: String,
    pub event_id: String,
}

static E2E_STATE: OnceCell<Arc<E2eState>> = OnceCell::const_new();
static HANDLER_HITS: AtomicUsize = AtomicUsize::new(0);

/// Lab topic registered into the in-process Photon registry.
pub const E2E_TOPIC_NAME: &str = "test.photon.e2e.ops";
/// Durable handler name (registry key suffix).
pub const E2E_HANDLER_NAME: &str = "e2e-ops-handler";

#[topic(name = "test.photon.e2e.ops")]
pub struct E2eOpsEvent {
    pub value: u32,
}

#[subscribe(topic = "test.photon.e2e.ops", durable = "e2e-ops-handler")]
async fn on_e2e_ops(_actor: Box<dyn Actor>, event: E2eOpsEvent) -> photon::Result<()> {
    assert_eq!(event.value, 42);
    HANDLER_HITS.fetch_add(1, Ordering::SeqCst);
    Ok(())
}

struct HiggsFactory(RouterValenceFactory);

impl HiggsValenceFactory for HiggsFactory {
    fn build(&self, actor_json: &serde_json::Value) -> anyhow::Result<Valence> {
        self.0.build(actor_json).map_err(|e| anyhow::anyhow!("{e}"))
    }
}

fn prepare_env() {
    valence::deletion::register_noop_deletion_dispatcher_for_tests();
    valence::clear_for_test();
    // SAFETY: host boot only.
    unsafe {
        if std::env::var_os("VALENCE_OWNERSHIP_UNIFIED_FETCH").is_none() {
            std::env::set_var("VALENCE_OWNERSHIP_UNIFIED_FETCH", "0");
        }
    }
}

fn ensure_transport_key() {
    if std::env::var_os("PHOTON_TRANSPORT_KEY").is_none() {
        // SAFETY: lab process boot only.
        unsafe {
            std::env::set_var(
                "PHOTON_TRANSPORT_KEY",
                // base64 of 32 zero bytes — accepted when ALLOW_DEV is set
                "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
            );
            std::env::set_var("PHOTON_ALLOW_DEV_TRANSPORT_KEY", "1");
        }
    }
}

async fn seed_user(id: &str, email_verified: bool, valence: &Valence) {
    let now = Utc::now();
    let confirmed_at = email_verified.then_some(now);
    let user = lepton::generated::User::new(
        Some(lepton::generated::UserUserType::Person),
        Some("e2e-password-hash".to_string()),
        Some(lepton::generated::UserStatus::Active),
        None,
        None,
        confirmed_at,
        None,
        None,
        now,
        now,
    )
    .expect("build user");
    lepton::generated::User::upsert(id, user, valence)
        .await
        .expect("upsert user");
}

async fn seed_super_user_with_member(system: &Valence, member_user_id: &str) {
    let super_group = gauge::generated::PermissionGroup::new(
        SUPER_USER_GROUP_NAME.to_string(),
        Some("super users".to_string()),
        Utc::now(),
        Utc::now(),
    )
    .expect("build super user group");
    let created =
        gauge::generated::PermissionGroup::upsert("super_user_group", super_group, system)
            .await
            .expect("upsert super user group");

    let member = lepton::generated::User::get(member_user_id, system)
        .await
        .expect("query member")
        .expect("member exists");
    let principal = gauge::generated::PermissionUserPrincipal::upsert(
        &format!("user:{member_user_id}"),
        gauge::generated::PermissionUserPrincipal::new(
            member.id().expect("member id").clone(),
            member_user_id.to_string(),
        )
        .expect("new principal"),
        system,
    )
    .await
    .expect("upsert principal");
    created
        .relate_to_owner_record(principal.id().expect("principal id"), system)
        .await
        .expect("relate super owner");
    created
        .relate_to_member_record(principal.id().expect("principal id"), system)
        .await
        .expect("relate super member");
}

async fn demote_admin_from_super_user(system: &Valence) {
    let Some(super_group) = gauge::generated::PermissionGroup::get("super_user_group", system)
        .await
        .expect("get super user group")
    else {
        return;
    };
    let Some(principal) = gauge::generated::PermissionUserPrincipal::get("user:admin", system)
        .await
        .expect("get admin principal")
    else {
        return;
    };
    let pid = principal.id().expect("principal id").clone();
    let _ = super_group.unrelate_from_member_record(&pid, system).await;
    let _ = super_group.unrelate_from_owner_record(&pid, system).await;
}

fn photon_admin_manifest() -> PermissionManifestInput {
    PermissionManifestInput {
        app_id: "photon".into(),
        domains: vec![PermissionDomainInput {
            key: "photon".into(),
            name: "Photon".into(),
            description: "Photon event pipeline administration".into(),
            permissions: vec![PermissionInput {
                name: "PhotonAdmin".into(),
                description: "Administer Photon topics, subscriptions, and event inspection".into(),
            }],
        }],
    }
}

async fn grant_photon_admin(admin_ctx: &Valence, user_id: &str) {
    let perms = service::list_permissions(admin_ctx, None)
        .await
        .expect("list permissions");
    let photon_admin = perms
        .into_iter()
        .find(|p| p.name == "PhotonAdmin")
        .expect("PhotonAdmin after sync");
    service::grant_permission_to_user(&photon_admin.id, user_id, admin_ctx)
        .await
        .expect("grant PhotonAdmin");
}

async fn bootstrap_photon_fixtures(photon: &Photon) -> anyhow::Result<FixtureIds> {
    HANDLER_HITS.store(0, Ordering::SeqCst);

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    E2eOpsEvent { value: 42 }.publish().await?;
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    photon
        .runtime()
        .executor_services
        .checkpoint_coalescer
        .flush()
        .await?;

    if HANDLER_HITS.load(Ordering::SeqCst) < 1 {
        anyhow::bail!("durable e2e handler must run");
    }

    let recent = photon_backend::ops::load_recent_events(photon, 50).await?;
    let event_id = recent
        .first()
        .map(|e| e.event_id.clone())
        .ok_or_else(|| anyhow::anyhow!("published event missing from recent list"))?;

    Ok(FixtureIds {
        topic_name: E2E_TOPIC_NAME.into(),
        subscription_id: format!("{E2E_TOPIC_NAME}:{E2E_HANDLER_NAME}"),
        event_id,
    })
}

/// Build shared Valence/Higgs/Photon once and seed baseline fixtures.
pub async fn init_e2e_valence() {
    E2E_STATE
        .get_or_init(|| async {
            prepare_env();
            ensure_transport_key();

            let backend: Arc<dyn DatabaseBackend> = Arc::new(InMemoryBackend::new());
            let mut router = DatabaseRouter::new();
            register_backend_logical_names(
                &mut router,
                Arc::clone(&backend),
                gauge::embedded_surreal::EMBEDDED_SURREAL_LOGICAL_NAMES,
                RegisterBackendLogicalNamesOptions {
                    register_alias_engine_id: Some(SQLITE_ENGINE_ID),
                },
            );
            router.register(
                router_key(gauge::embedded_surreal::LOGICAL_NAME, SQLITE_ENGINE_ID),
                Arc::clone(&backend),
            );
            let router = Arc::new(router);
            let default_key = router_key(gauge::embedded_surreal::LOGICAL_NAME, MEM_ENGINE_ID);

            let system = Valence::builder()
                .database_router(Arc::clone(&router))
                .default_backend_key(default_key.clone())
                .with_actor(ValenceActor::System {
                    operation: "e2e_photon_host".into(),
                })
                .build()
                .expect("e2e Valence");

            seed_user("admin", true, &system).await;
            seed_user("outsider", true, &system).await;
            seed_user("unverified", false, &system).await;
            seed_super_user_with_member(&system, "admin").await;

            sync_permission_manifests(&system, &[photon_admin_manifest()])
                .await
                .expect("sync PhotonAdmin manifest");

            let admin_ctx = system.with_actor(ValenceActor::User {
                user_id: "admin".to_string(),
            });
            grant_photon_admin(&admin_ctx, "admin").await;
            grant_photon_admin(&admin_ctx, "unverified").await;
            demote_admin_from_super_user(&system).await;

            let photon = Photon::builder()
                .auto_registry()
                .build()
                .expect("e2e Photon");
            photon
                .start_executor(Arc::new(JsonIdentityFactory))
                .expect("start executor");
            configure(photon.clone());

            let fixtures = bootstrap_photon_fixtures(&photon)
                .await
                .expect("bootstrap photon fixtures");
            let photon = Arc::new(photon);

            let factory: Arc<dyn HiggsValenceFactory> =
                Arc::new(HiggsFactory(RouterValenceFactory::new(
                    Arc::clone(&router),
                    RouterValenceFactoryConfig::new(default_key.clone())
                        .actor_json_policy(external_actor_json_policy()),
                )));
            let higgs = Arc::new(
                HiggsConfig::builder()
                    .valence_factory_arc(factory)
                    .build()
                    .expect("e2e HiggsConfig"),
            );

            Arc::new(E2eState {
                router,
                higgs,
                photon,
                default_backend_key: default_key,
                fixtures: Mutex::new(fixtures),
            })
        })
        .await;
}

fn state() -> Arc<E2eState> {
    E2E_STATE
        .get()
        .expect("init_e2e_valence must run first")
        .clone()
}

pub fn e2e_router() -> Arc<DatabaseRouter> {
    Arc::clone(&state().router)
}

pub fn e2e_higgs_config() -> Arc<HiggsConfig> {
    Arc::clone(&state().higgs)
}

pub fn e2e_photon() -> Arc<Photon> {
    Arc::clone(&state().photon)
}

pub fn e2e_fixtures() -> FixtureIds {
    state().fixtures.lock().expect("fixtures").clone()
}

pub fn e2e_system_valence() -> Valence {
    Valence::builder()
        .database_router(e2e_router())
        .default_backend_key(state().default_backend_key.clone())
        .with_actor(ValenceActor::System {
            operation: "e2e_seed".into(),
        })
        .build()
        .expect("system valence")
}
