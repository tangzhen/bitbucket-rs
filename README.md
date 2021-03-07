# bitbucket-rs
A Rust library implementing the Bitbucket/Stash Rest API

## Overview
**NOTE:** This is work in progress.


bitbucket-rs is a Rust library that implement the Bitbucket/Stash Rest API as per [this documentation](https://docs.atlassian.com/DAC/rest/stash/3.11.3/stash-rest.html).
This library provides various components for use and might be split into more, smaller libraries in the future.\
The purpose of this client, is to provide the implementation for the standard Bitbucket operations.\
In that way, implementing a Bitbucket application client using this library should be quite trivial.

Of course, you can build upon the functionality of this library and provide more features based on this information  such as filtering open pull requests for a target branch based on date.

## Components
This library provides the following components that you can use them as it they are, or use some of them to\
implement your library if you want.
- models: Includes all the structures that you can have when getting a response or use when sending a request
- resources: Includes all the available resources that you can retrieve
- uri_builders: Includes helper structures based on the builder pattern, which allow you to construct all the available Rest URIs available
- traits: Includes traits that you can implement yourself in order to interface with the components
- client: A client implementation that can send HTTP requests and deserialize responses to the the available models

