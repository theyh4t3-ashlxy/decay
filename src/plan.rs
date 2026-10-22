struct Ideas {
    public_schools: bool,
    prussian: bool,
}

fn main() {
    let jets = Ideas {
        public_schools: true,
        prussian: true,
    };

    if jets.prussian {
        println!("compulsory.");
    }

    if jets.public_schools {
        println!("is it really public? or is it just charter disguised as public to convince?");
    }
}