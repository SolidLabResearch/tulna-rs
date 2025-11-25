use tulna_rs::isomorphism::api::QueryIsomorphismAPI;

fn main() {
    println!("=== Query Isomorphism Examples ===\n");

    // Example 1: Simple SPARQL query isomorphism
    println!("Example 1: SPARQL Query Isomorphism");
    let sparql1 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o . }";
    let sparql2 = "SELECT ?x ?y ?z WHERE { ?x ?y ?z . }";

    match QueryIsomorphismAPI::is_isomorphic(sparql1, sparql2) {
        Ok(true) => println!("Queries are isomorphic (variables renamed)"),
        Ok(false) => println!("Queries are NOT isomorphic"),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Example 2: Non-isomorphic SPARQL queries
    println!("Example 2: Non-Isomorphic SPARQL Queries");
    let sparql3 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o . }";
    let sparql4 = "SELECT ?s ?p ?o WHERE { ?s <http://example.org/different> ?o . }";

    match QueryIsomorphismAPI::is_isomorphic(sparql3, sparql4) {
        Ok(true) => println!("Queries are isomorphic"),
        Ok(false) => println!("Queries are NOT isomorphic (different predicates)"),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Example 3: Detect query language
    println!("Example 3: Query Language Detection");
    let queries = vec![
        ("SPARQL", "SELECT ?s ?p ?o WHERE { ?s ?p ?o . }"),
        (
            "RSPQL",
            "REGISTER RStream <output> AS SELECT ?s ?p ?o FROM NAMED WINDOW <w> ON STREAM <s> [RANGE 10 STEP 5] WHERE { WINDOW <w> { ?s ?p ?o } }",
        ),
        (
            "JanusQL",
            "REGISTER RStream <output> AS SELECT ?s ?p ?o FROM NAMED WINDOW <w> ON STREAM <s> [OFFSET 0 RANGE 10 STEP 5] WHERE { WINDOW <w> { ?s ?p ?o } }",
        ),
    ];

    for (expected, query) in queries {
        let detected = QueryIsomorphismAPI::detect_query_language(query);
        println!("Expected: {}, Detected: {:?}", expected, detected);
    }
    println!();

    // Example 4: Extract BGP from a query
    println!("Example 4: Extract Basic Graph Pattern");
    let query_with_bgp =
        "SELECT ?person ?name WHERE { ?person <http://xmlns.com/foaf/0.1/name> ?name . }";

    match QueryIsomorphismAPI::extract_bgp(query_with_bgp) {
        Ok(bgp) => {
            println!("Extracted {} triple(s) from BGP:", bgp.len());
            for (i, triple) in bgp.iter().enumerate() {
                println!(
                    "  Triple {}: {:?} {:?} {:?}",
                    i + 1,
                    triple.subject,
                    triple.predicate,
                    triple.object
                );
            }
        }
        Err(e) => println!("Error extracting BGP: {}", e),
    }
    println!();

    // Example 5: RSPQL query isomorphism
    println!("Example 5: RSPQL Query Isomorphism");
    let rspql1 = r#"
PREFIX : <http://example.org/>
REGISTER RStream <output> AS
SELECT ?s ?p ?o
FROM NAMED WINDOW :w ON STREAM :stream [RANGE 10 STEP 5]
WHERE {
    WINDOW :w { ?s ?p ?o }
}
"#;

    let rspql2 = r#"
PREFIX : <http://example.org/>
REGISTER RStream <output> AS
SELECT ?x ?y ?z
FROM NAMED WINDOW :w ON STREAM :stream [RANGE 10 STEP 5]
WHERE {
    WINDOW :w { ?x ?y ?z }
}
"#;

    match QueryIsomorphismAPI::is_isomorphic(rspql1, rspql2) {
        Ok(true) => println!("RSPQL queries are isomorphic"),
        Ok(false) => println!("RSPQL queries are NOT isomorphic"),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Example 6: Detailed query comparison
    println!("Example 6: Detailed Query Comparison");
    let q1 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o . }";
    let q2 = "SELECT ?a ?b ?c WHERE { ?a ?b ?c . }";

    match QueryIsomorphismAPI::compare_queries(q1, q2) {
        Ok(comparison) => {
            println!("{}", comparison.summary());
        }
        Err(e) => println!("Error comparing queries: {}", e),
    }
    println!();

    // Example 7: Check stream parameters
    println!("Example 7: Check Stream Parameters");
    let rspql_a = r#"
REGISTER RStream <output> AS
SELECT ?s ?p ?o
FROM NAMED WINDOW <w> ON STREAM <mystream> [RANGE 10 STEP 5]
WHERE { WINDOW <w> { ?s ?p ?o } }
"#;

    let rspql_b = r#"
REGISTER RStream <output> AS
SELECT ?s ?p ?o
FROM NAMED WINDOW <w> ON STREAM <mystream> [RANGE 10 STEP 5]
WHERE { WINDOW <w> { ?s ?p ?o } }
"#;

    match QueryIsomorphismAPI::check_stream_parameters(rspql_a, rspql_b) {
        Ok(true) => println!("Stream parameters match"),
        Ok(false) => println!("Stream parameters do NOT match"),
        Err(e) => println!("Error: {}", e),
    }
    println!();

    // Example 8: Parse and inspect a query
    println!("Example 8: Parse and Inspect Query");
    let complex_query = r#"
PREFIX ex: <http://example.org/>
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?person ?name ?email
WHERE {
    ?person foaf:name ?name .
    ?person foaf:mbox ?email .
}
"#;

    match QueryIsomorphismAPI::parse_query(complex_query) {
        Ok(parsed) => {
            println!("Query Language: {:?}", parsed.query_language);
            println!("Number of triples in BGP: {}", parsed.bgp.len());
            println!("Stream Name: {:?}", parsed.stream_name);
            println!("Window Name: {:?}", parsed.window_name);
        }
        Err(e) => println!("Error parsing query: {}", e),
    }
}
