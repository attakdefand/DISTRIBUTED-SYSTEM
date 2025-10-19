use rustls::{ClientConfig, ServerConfig, RootCertStore};
use std::sync::Arc;
use webpki::TrustAnchor;

pub struct TlsConfig {
    pub client_config: Arc<ClientConfig>,
    pub server_config: Option<Arc<ServerConfig>>,
}

impl TlsConfig {
    pub fn new() -> Result<Self, crate::SecurityError> {
        // Create a root certificate store
        let mut root_store = RootCertStore::empty();
        
        // In a real implementation, you would add trusted certificates here
        // For now, we'll create a basic config without specific certificates
        
        // Create client config
        let client_config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(root_store)
            .with_no_client_auth();
            
        Ok(Self {
            client_config: Arc::new(client_config),
            server_config: None,
        })
    }
    
    pub fn with_server_certificates(
        &mut self,
        cert_chain: Vec<rustls::Certificate>,
        private_key: rustls::PrivateKey,
    ) -> Result<&mut Self, crate::SecurityError> {
        let server_config = ServerConfig::builder()
            .with_safe_defaults()
            .with_no_client_auth()
            .with_single_cert(cert_chain, private_key)
            .map_err(|e| crate::SecurityError::TlsError(e.to_string()))?;
            
        self.server_config = Some(Arc::new(server_config));
        Ok(self)
    }
}