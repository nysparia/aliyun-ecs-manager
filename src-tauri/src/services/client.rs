use alibabacloud::client::AliyunClient;
use std::sync::RwLock;

/// Service for managing Aliyun client instances.
///
/// This service uses `RwLock` to ensure thread-safety while allowing multiple concurrent readers.
/// The client is wrapped in an `Option` to handle scenarios where valid credentials may not be
/// available at application startup.
pub struct AliyunClientService {
    /// The Aliyun client instance, wrapped in `Option` to support lazy initialization.
    client: RwLock<Option<AliyunClient>>,
}

impl AliyunClientService {
    /// Creates a new `AliyunClientService` instance with no client initialized.
    pub fn new() -> Self {
        Self {
            client: RwLock::new(None),
        }
    }

    /// Initializes or updates the Aliyun client instance.
    ///
    /// This method can be called at application startup or whenever credentials need to be updated.
    ///
    /// # Arguments
    ///
    /// * `client` - The configured `AliyunClient` instance to store.
    pub fn initialize(&self, client: AliyunClient) {
        let mut guard = self.client.write().unwrap();
        *guard = Some(client);
    }

    /// Executes a closure with a reference to the client if it's initialized.
    ///
    /// This method provides safe access to the client instance through a callback function.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a reference to `AliyunClient` and returns a value of type `R`.
    ///
    /// # Returns
    ///
    /// * `Some(R)` - If the client is initialized, returns the result of the closure.
    /// * `None` - If the client is not initialized or credentials are invalid.
    pub fn with_client<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&AliyunClient) -> R,
    {
        let guard = self.client.read().unwrap();
        guard.as_ref().map(f)
    }

    /// Checks whether the client has been initialized.
    ///
    /// # Returns
    ///
    /// * `true` - If the client is initialized and available.
    /// * `false` - If the client has not been initialized.
    pub fn is_initialized(&self) -> bool {
        self.client.read().unwrap().is_some()
    }

    /// Clears the client instance.
    ///
    /// This method should be called when logging out or when credentials need to be invalidated.
    pub fn clear(&self) {
        let mut guard = self.client.write().unwrap();
        *guard = None;
    }
}