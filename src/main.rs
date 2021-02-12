extern crate vps_bench;

use structopt::StructOpt;

fn main() {
    let opt: vps_bench::BenchCli = vps_bench::BenchCli::from_args();

    opt.run();
}
