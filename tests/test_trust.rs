mod test_utils;

use httpmock::MockServer;
use insta_cmd::assert_cmd_snapshot;
use test_utils::{cmd, create_mock_server, fixture_path, test_img_path};

#[test]
fn test_load_trust_from_trusted_file() {
    assert_cmd_snapshot!(cmd()
        .arg("view")
        .arg("manifest")
        .arg(test_img_path())
        .arg("--trust-anchors")
        .arg(fixture_path("trust/anchors.pem"))
        .arg("--trust-config")
        .arg(fixture_path("trust/store.cfg")));
}

#[test]
fn test_load_trust_from_untrusted_file() {
    assert_cmd_snapshot!(cmd()
        .arg("view")
        .arg("manifest")
        .arg(test_img_path())
        .arg("--trust-anchors")
        .arg(fixture_path("trust/no-match.pem"))
        .arg("--trust-config")
        .arg(fixture_path("trust/store.cfg")));
}

#[test]
fn test_load_trust_from_trusted_url() {
    let server = MockServer::start();
    let mocks = create_mock_server(&server, "trust/anchors.pem", "trust/store.cfg");

    assert_cmd_snapshot!(cmd()
        .arg("view")
        .arg("manifest")
        .arg(test_img_path())
        .arg("--trust-anchors")
        .arg(server.url("/trust/anchors.pem"))
        .arg("--trust-config")
        .arg(server.url("/trust/store.cfg")));

    mocks.iter().for_each(|m| m.assert());
}

#[test]
fn test_load_trust_from_untrusted_url() {
    let server = MockServer::start();
    let mocks = create_mock_server(&server, "trust/no-match.pem", "trust/store.cfg");

    assert_cmd_snapshot!(cmd()
        .arg("view")
        .arg("manifest")
        .arg(test_img_path())
        .arg("--trust-anchors")
        .arg(server.url("/trust/anchors.pem"))
        .arg("--trust-config")
        .arg(server.url("/trust/store.cfg")));

    mocks.iter().for_each(|m| m.assert());
}

#[test]
fn test_load_trust_from_trusted_url_env() {
    let server = MockServer::start();
    let mocks = create_mock_server(&server, "trust/anchors.pem", "trust/store.cfg");

    assert_cmd_snapshot!(cmd()
        .arg("view")
        .arg("manifest")
        .arg(test_img_path())
        .env("C2PATOOL_TRUST_ANCHORS", server.url("/trust/anchors.pem"))
        .env("C2PATOOL_TRUST_CONFIG", server.url("/trust/store.cfg")));

    mocks.iter().for_each(|m| m.assert());
}

#[test]
fn test_load_trust_from_untrusted_url_env() {
    let server = MockServer::start();
    let mocks = create_mock_server(&server, "trust/no-match.pem", "trust/store.cfg");

    assert_cmd_snapshot!(cmd()
        .arg("view")
        .arg("manifest")
        .arg(test_img_path())
        .env("C2PATOOL_TRUST_ANCHORS", server.url("/trust/anchors.pem"))
        .env("C2PATOOL_TRUST_CONFIG", server.url("/trust/store.cfg")));

    mocks.iter().for_each(|m| m.assert());
}