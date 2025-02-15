use crate::integration::{
    assert_endpoints, mock_common_upload_endpoints, mock_endpoint, register_test, EndpointOptions,
    ServerBehavior,
};

#[test]
fn command_sourcemaps_upload_help() {
    register_test("sourcemaps/sourcemaps-upload-help.trycmd");
}

#[test]
fn command_sourcemaps_upload() {
    register_test("sourcemaps/sourcemaps-upload.trycmd");
}

#[test]
fn command_sourcemaps_upload_successfully_upload_file() {
    let upload_endpoints = mock_common_upload_endpoints(ServerBehavior::Legacy);
    let _files = mock_endpoint(
        EndpointOptions::new(
            "GET",
            "/api/0/projects/wat-org/wat-project/releases/wat-release/files/?cursor=",
            200,
        )
        .with_response_body("[]"),
    );

    register_test("sourcemaps/sourcemaps-upload-successfully-upload-file.trycmd");
    assert_endpoints(&upload_endpoints);
}

#[test]
fn command_sourcemaps_upload_skip_already_uploaded() {
    let upload_endpoints = mock_common_upload_endpoints(ServerBehavior::Legacy);
    let _files = mock_endpoint(
        EndpointOptions::new(
            "GET",
            "/api/0/projects/wat-org/wat-project/releases/wat-release/files/?cursor=&checksum=38ed853073df85147960ea3a5bced6170ec389b0&checksum=f3673e2cea68bcb86bb74254a9efaa381d74929f",
            200,
        )
        .with_response_body(
            r#"[{
                "id": "1337",
                "name": "~/vendor.min.js.map",
                "headers": {},
                "size": 1522,
                "sha1": "f3673e2cea68bcb86bb74254a9efaa381d74929f",
                "dateCreated": "2022-05-12T11:08:01.496220Z"
            }]"#,
        ),
    );

    register_test("sourcemaps/sourcemaps-upload-skip-already-uploaded.trycmd");
    assert_endpoints(&upload_endpoints);
}

#[test]
fn command_sourcemaps_upload_no_dedupe() {
    let upload_endpoints = mock_common_upload_endpoints(ServerBehavior::Legacy);
    let _files = mock_endpoint(
        EndpointOptions::new(
            "GET",
            "/api/0/projects/wat-org/wat-project/releases/wat-release/files/?cursor=",
            200,
        )
        .with_response_body(
            r#"[{
                "id": "1337",
                "name": "~/vendor.min.js.map",
                "headers": {},
                "size": 1522,
                "sha1": "f3673e2cea68bcb86bb74254a9efaa381d74929f",
                "dateCreated": "2022-05-12T11:08:01.496220Z"
            }]"#,
        ),
    );

    register_test("sourcemaps/sourcemaps-upload-no-dedupe.trycmd");
    assert_endpoints(&upload_endpoints);
}

#[test]
fn command_sourcemaps_upload_modern() {
    let upload_endpoints = mock_common_upload_endpoints(ServerBehavior::Modern);
    register_test("sourcemaps/sourcemaps-upload-modern.trycmd");
    assert_endpoints(&upload_endpoints);
}

#[test]
fn command_sourcemaps_upload_empty() {
    let _upload_endpoints = mock_common_upload_endpoints(ServerBehavior::Legacy);
    let _files = mock_endpoint(
        EndpointOptions::new(
            "GET",
            "/api/0/projects/wat-org/wat-project/releases/wat-release/files/?cursor=",
            200,
        )
        .with_response_body("[]"),
    );
    register_test("releases/releases-files-upload-sourcemaps.trycmd");
}

#[test]
fn command_sourcemaps_upload_some_debugids() {
    let upload_endpoints = mock_common_upload_endpoints(ServerBehavior::Modern);
    register_test("sourcemaps/sourcemaps-upload-some-debugids.trycmd");
    assert_endpoints(&upload_endpoints);
}

#[test]
fn command_sourcemaps_upload_no_debugids() {
    let _upload_endpoints = mock_common_upload_endpoints(ServerBehavior::Modern);
    register_test("sourcemaps/sourcemaps-upload-no-debugids.trycmd");
}
