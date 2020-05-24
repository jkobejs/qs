#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
       assert_eq!(2 + 2, 4);
    }
}

trait QueryString<T> {
    fn decode(query_string: &str) -> Option<T>;
    fn encode(t: &T) -> &str;

    // ?size=1&name=josip (1, "josip")
    // fn combine<V>(&self, &qs: QueryString<V>) -> &QueryString<(V, T)>;
}




