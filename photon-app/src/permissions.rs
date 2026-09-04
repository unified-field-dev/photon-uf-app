//! Permission manifest for the Photon operations app.

use uf_product_macros::UfPermissionManifest;

/// Admin permission for Photon ops UI server functions.
///
/// Synced into the `photon` domain; gated with
/// `#[uf_product_macros::server(permission = "PhotonAdmin")]`.
#[allow(clippy::expl_impl_clone_on_copy)]
#[derive(UfPermissionManifest)]
#[permission_manifest(
    domain_key = "photon",
    domain_name = "Photon",
    domain_description = "Photon event pipeline administration"
)]
pub enum PhotonPermission {
    /// Read topics, subscriptions, events, and dashboard aggregates.
    #[permission(description = "Administer Photon topics, subscriptions, and event inspection")]
    PhotonAdmin,
}
