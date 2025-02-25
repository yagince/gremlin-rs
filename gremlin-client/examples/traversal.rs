use gremlin_client::{process::traversal::traversal, GremlinClient};

fn main() -> Result<(), Box<std::error::Error>> {
    let client = GremlinClient::connect("localhost")?;

    let g = traversal().with_remote(client);

    let vertices = g
        .v(())
        .has_label("person")
        .has(("name", "marko"))
        .to_list()?;

    println!("{:?}", vertices);

    let friends = g
        .v(())
        .has_label("person")
        .has(("name", "marko"))
        .out("knows")
        .to_list()?;

    println!("{:?}", friends);

    Ok(())
}
