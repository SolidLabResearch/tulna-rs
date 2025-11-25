use tulna_rs::isomorphism::api::QueryIsomorphismAPI;

#[test]
fn test_simple_sparql_isomorphism() {
    let query1 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o . }";
    let query2 = "SELECT ?x ?y ?z WHERE { ?x ?y ?z . }";

    let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_sparql_different_variable_names() {
    let query1 = r#"
SELECT ?person ?name
WHERE {
    ?person <http://xmlns.com/foaf/0.1/name> ?name .
}
"#;

    let query2 = r#"
SELECT ?x ?y
WHERE {
    ?x <http://xmlns.com/foaf/0.1/name> ?y .
}
"#;

    let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_sparql_with_prefixes() {
    let query1 = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?person ?name
WHERE {
    ?person foaf:name ?name .
}
"#;

    let query2 = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?x ?y
WHERE {
    ?x foaf:name ?y .
}
"#;

    let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_sparql_multiple_triples_isomorphic() {
    let query1 = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?person ?name ?email
WHERE {
    ?person foaf:name ?name .
    ?person foaf:mbox ?email .
}
"#;

    let query2 = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?x ?y ?z
WHERE {
    ?x foaf:name ?y .
    ?x foaf:mbox ?z .
}
"#;

    let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_sparql_not_isomorphic_different_predicates() {
    let query1 = r#"
SELECT ?s ?o
WHERE {
    ?s <http://example.org/predicate1> ?o .
}
"#;

    let query2 = r#"
SELECT ?s ?o
WHERE {
    ?s <http://example.org/predicate2> ?o .
}
"#;

    let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_sparql_not_isomorphic_different_structure() {
    let query1 = r#"
PREFIX ex: <http://example.org/>
SELECT ?x ?y
WHERE {
    ?x ex:knows ?y .
}
"#;

    let query2 = r#"
PREFIX ex: <http://example.org/>
SELECT ?x ?y ?z
WHERE {
    ?x ex:knows ?y .
    ?y ex:knows ?z .
}
"#;

    let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_sparql_chain_pattern_isomorphic() {
    let query1 = r#"
PREFIX ex: <http://example.org/>
SELECT ?x ?y ?z
WHERE {
    ?x ex:knows ?y .
    ?y ex:knows ?z .
}
"#;

    let query2 = r#"
PREFIX ex: <http://example.org/>
SELECT ?a ?b ?c
WHERE {
    ?a ex:knows ?b .
    ?b ex:knows ?c .
}
"#;

    let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_sparql_with_rdf_type() {
    let query1 = r#"
PREFIX ex: <http://example.org/>
SELECT ?person
WHERE {
    ?person a ex:Person .
}
"#;

    let query2 = r#"
PREFIX ex: <http://example.org/>
SELECT ?x
WHERE {
    ?x a ex:Person .
}
"#;

    let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_sparql_complex_pattern() {
    let query1 = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX ex: <http://example.org/>
SELECT ?person ?friend ?friendName
WHERE {
    ?person a ex:Person .
    ?person foaf:knows ?friend .
    ?friend foaf:name ?friendName .
}
"#;

    let query2 = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
PREFIX ex: <http://example.org/>
SELECT ?p ?f ?n
WHERE {
    ?p a ex:Person .
    ?p foaf:knows ?f .
    ?f foaf:name ?n .
}
"#;

    let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_sparql_with_literal() {
    let query1 = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?person
WHERE {
    ?person foaf:name "Alice" .
}
"#;

    let query2 = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?x
WHERE {
    ?x foaf:name "Alice" .
}
"#;

    let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_sparql_not_isomorphic_different_literal() {
    let query1 = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?person
WHERE {
    ?person foaf:name "Alice" .
}
"#;

    let query2 = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?person
WHERE {
    ?person foaf:name "Bob" .
}
"#;

    let result = QueryIsomorphismAPI::is_isomorphic(query1, query2);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn test_sparql_detect_language() {
    let query = "SELECT ?s ?p ?o WHERE { ?s ?p ?o . }";
    let language = QueryIsomorphismAPI::detect_query_language(query);

    assert_eq!(language, tulna_rs::isomorphism::core::QueryLanguage::SPARQL);
}

#[test]
fn test_sparql_extract_bgp() {
    let query = r#"
PREFIX foaf: <http://xmlns.com/foaf/0.1/>
SELECT ?person ?name
WHERE {
    ?person foaf:name ?name .
}
"#;

    let bgp = QueryIsomorphismAPI::extract_bgp(query);
    assert!(bgp.is_ok());
    let triples = bgp.unwrap();
    assert_eq!(triples.len(), 1);
}

#[test]
fn test_sparql_compare_queries_detailed() {
    let query1 = "SELECT ?s ?p ?o WHERE { ?s ?p ?o . }";
    let query2 = "SELECT ?x ?y ?z WHERE { ?x ?y ?z . }";

    let comparison = QueryIsomorphismAPI::compare_queries(query1, query2);
    assert!(comparison.is_ok());

    let result = comparison.unwrap();
    assert!(result.is_isomorphic);
    assert!(result.same_language);
    assert!(result.same_bgp_size);
    assert!(result.bgp_isomorphic);
}
