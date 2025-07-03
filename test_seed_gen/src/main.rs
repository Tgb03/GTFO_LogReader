use gtfo_log_reader_core::{
    output_trait::{OutputTrait, PrintOutput},
    seed_gen::unity_random::UnityRandom,
};

fn main() {
    let mut ur = UnityRandom::from(732336958);

    for _ in 0..10 {
        let v = ur.next().unwrap();

        let p = PrintOutput;
        p.output(v);
    }

    for _ in 0..1014 {
        let _ = ur.next().unwrap();
    }

    println!("^");
    println!("| ...1014 seeds");
    println!("v");

    for _ in 0..10 {
        let v = ur.next().unwrap();

        let p = PrintOutput;
        p.output(v);
    }
}
