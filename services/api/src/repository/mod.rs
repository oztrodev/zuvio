pub mod in_memory;
pub mod traits;

pub use in_memory::InMemoryRepository;
pub use traits::{RepoError, Repository};

