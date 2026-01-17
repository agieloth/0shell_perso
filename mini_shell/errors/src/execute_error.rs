use std::io;

#[derive(Debug)]
pub enum ExecuteError {
    // Erreurs pour cd
    CdNoHome,
    CdFailed(String, io::Error),
    
    // Erreurs pour pwd
    PwdFailed(io::Error),
    
    // Erreurs pour echo (pas vraiment d'erreurs)
    // Erreurs pour cat
    CatFileNotFound(String),
    CatReadFailed(String, io::Error),
    CatNoArguments,
    
    // Erreurs pour mkdir
    MkdirNoArguments,
    MkdirFailed(String, io::Error),
    
    // Erreurs pour cp
    CpInvalidArguments,
    CpFailed(String, String, io::Error),
    
    // Erreurs pour mv
    MvInvalidArguments,
    MvFailed(String, String, io::Error),
    
    // Erreurs pour rm
    RmNoArguments,
    RmFailed(String, io::Error),
    
    // Erreurs pour ls
    LsFailed(String, io::Error),
}

impl std::fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // cd
            ExecuteError::CdNoHome => write!(f, "cd: HOME directory not found"),
            ExecuteError::CdFailed(path, e) => write!(f, "cd: {}: {}", path, e),
            
            // pwd
            ExecuteError::PwdFailed(e) => write!(f, "pwd: {}", e),
            
            // cat
            ExecuteError::CatNoArguments => write!(f, "cat: missing file operand"),
            ExecuteError::CatFileNotFound(file) => write!(f, "cat: {}: No such file or directory", file),
            ExecuteError::CatReadFailed(file, e) => write!(f, "cat: {}: {}", file, e),
            
            // mkdir
            ExecuteError::MkdirNoArguments => write!(f, "mkdir: missing operand"),
            ExecuteError::MkdirFailed(dir, e) => write!(f, "mkdir: cannot create directory '{}': {}", dir, e),
            
            // cp
            ExecuteError::CpInvalidArguments => write!(f, "cp: missing file operand"),
            ExecuteError::CpFailed(src, dest, e) => write!(f, "cp: cannot copy '{}' to '{}': {}", src, dest, e),
            
            // mv
            ExecuteError::MvInvalidArguments => write!(f, "mv: missing file operand"),
            ExecuteError::MvFailed(src, dest, e) => write!(f, "mv: cannot move '{}' to '{}': {}", src, dest, e),
            
            // rm
            ExecuteError::RmNoArguments => write!(f, "rm: missing operand"),
            ExecuteError::RmFailed(path, e) => write!(f, "rm: cannot remove '{}': {}", path, e),
            
            // ls
            ExecuteError::LsFailed(path, e) => write!(f, "ls: cannot access '{}': {}", path, e),
        }
    }
}

impl std::error::Error for ExecuteError {}