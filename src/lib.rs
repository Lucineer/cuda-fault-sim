//! cuda-fault-sim — GPU fault simulation

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FaultType { SA0, SA1, Bridging, Delay, Open }

#[derive(Debug, Clone)]
pub struct Fault { pub id: usize, pub net: String, pub ft: FaultType, pub detected: bool, pub count: u32 }

#[derive(Debug)]
pub struct Coverage { pub total: usize, pub detected: usize, pub pct: f64, pub undetected: Vec<String> }

pub struct FaultSim { pub nets: Vec<String>, pub faults: Vec<Fault>, pub counter: usize }

impl FaultSim {
    pub fn new() -> Self { FaultSim { nets: Vec::new(), faults: Vec::new(), counter: 0 } }
    pub fn add_nets(&mut self, names: &[&str]) { for n in names { self.nets.push(n.to_string()); } }
    pub fn inject_sa(&mut self) {
        for net in &self.nets { for ft in [FaultType::SA0, FaultType::SA1] {
            self.counter += 1; self.faults.push(Fault { id: self.counter, net: net.clone(), ft, detected: false, count: 0 });
        }}
    }
    pub fn run_scan(&mut self, patterns: &[[bool; 8]], n_inputs: usize) -> Coverage {
        for f in &mut self.faults { f.detected = false; f.count = 0; }
        for pattern in patterns { for (i, net) in self.nets.iter().enumerate() {
            let expected = i < n_inputs && pattern[i];
            if let Some(f) = self.faults.iter_mut().find(|f| f.net == *net && f.ft == FaultType::SA0) {
                if expected != false { f.detected = true; f.count += 1; }
            }
            if let Some(f) = self.faults.iter_mut().find(|f| f.net == *net && f.ft == FaultType::SA1) {
                if expected != true { f.detected = true; f.count += 1; }
            }
        }}
        let det = self.faults.iter().filter(|f| f.detected).count();
        let total = self.faults.len();
        Coverage { total, detected: det, pct: if total>0 {det as f64/total as f64*100.0} else {0.0},
            undetected: self.faults.iter().filter(|f| !f.detected).map(|f| format!("{}:{:?}", f.net, f.ft)).collect() }
    }
}

#[cfg(test)]
mod tests { use super::*;
    #[test] fn test_sa() { let mut s = FaultSim::new(); s.add_nets(&["a","b","c","d"]); s.inject_sa(); assert_eq!(s.faults.len(), 8); }
    #[test] fn test_scan() { let mut s = FaultSim::new(); s.add_nets(&["a","b","c","d"]); s.inject_sa();
        let patterns: Vec<[bool;8]> = (0..20).map(|i| [(i>>j)&1==1; 8]).collect();
        let cov = s.run_scan(&patterns, 4); assert!(cov.pct > 50.0); }
}
