{ pkgs
, connector-url
, engine-graphql-url
, service ? { } # additional options to customize this service configuration
}:

let
  repo-source-mount-point = "/src";

  integration-tests-service = {
    useHostStore = true;
    command = [
      "${pkgs.pkgsCross.linux.integration-tests}/bin/integration-tests"
    ];
    environment = {
      CONNECTOR_URL = connector-url;
      ENGINE_GRAPHQL_URL = engine-graphql-url;
      INSTA_WORKSPACE_ROOT = repo-source-mount-point;
      MONGODB_IMAGE = builtins.getEnv "MONGODB_IMAGE";
    };
    volumes = [
      "${builtins.getEnv "PWD"}:${repo-source-mount-point}:rw"
    ];
  };
in
{
  service =
    # merge service definition with overrides
    pkgs.lib.attrsets.recursiveUpdate integration-tests-service service;
}
