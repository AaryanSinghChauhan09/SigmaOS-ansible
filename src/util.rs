//! ansible model — configuration management and orchestration.

#[derive(Debug, Clone)]
pub struct Host {
    pub name: String,
    pub groups: Vec<String>,
    pub reachable: bool,
}

#[derive(Debug, Clone)]
pub struct Playbook {
    pub name: String,
    pub hosts: String,
    pub tasks: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub hosts: Vec<Host>,
    pub playbooks: Vec<Playbook>,
}

impl Default for Model {
    fn default() -> Self { Self { hosts: vec![], playbooks: vec![] } }
}

impl Model {
    pub fn new() -> Self { Self::default() }

    pub fn add_host(&mut self, host: Host) { self.hosts.push(host); }

    pub fn run_playbook(&mut self, name: &str) -> Option<&Playbook> {
        self.playbooks.iter().find(|p| p.name == name)
    }

    pub fn list_hosts(&self) -> &[Host] { &self.hosts }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_run() {
        let mut a = Model::new();
        a.add_host(Host { name: "web1".into(), groups: vec!["web".into()], reachable: true });
        a.playbooks.push(Playbook { name: "setup".into(), hosts: "web".into(), tasks: vec!["install nginx".into()] });
        assert!(a.run_playbook("setup").is_some());
    }
}
