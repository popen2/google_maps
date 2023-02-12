use crate::roads::snap_to_roads::{SERVICE_URL, request::Request};

// =============================================================================

impl<'a> Request<'a> {

    // -------------------------------------------------------------------------
    //
    /// Returns the URL query string that represents the query you've built.
    ///
    /// ## Description:
    ///
    /// Returns the query string that will be sent to the Google Maps API. It
    /// is the result of the builder pattern. This method could be useful for
    /// records or logging. It could also be used for passing to your HTTP
    /// client of choice and executing the HTTP GET request yourself.
    ///
    /// ## Arguments:
    ///
    /// This method accepts no arguments.

    pub fn query_url(&'a mut self) -> String {

        let query_string = match &self.query {
            // If query string has already been built, return it:
            Some(query_string) => query_string,
            // If it hasn't been built, build it:
            None => self.build().query.as_ref().unwrap(),
        }; // match

        format!("{SERVICE_URL}/?{query_string}")

    } // fn

} // impl