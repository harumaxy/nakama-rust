pub mod nakama {
    pub mod api {
        include!("gen/nakama.api.rs");
    }
    pub mod realtime {
        include!("gen/nakama.realtime.rs");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
