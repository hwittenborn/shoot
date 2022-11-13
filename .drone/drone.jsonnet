local deploy() = {
    name: "deploy",
    kind: "pipeline",
    type: "docker",
    trigger: {branch: ["main"]},
    steps: [
        {
            name: "run-tests",
            image: "proget.makedeb.org/docker/makedeb/makedeb:ubuntu-jammy",
            commands: [
                "sudo chown 'makedeb:makedeb' ./ -R",
                ".drone/scripts/setup-pbmpr.sh",
                "sudo apt-get install cargo -y",
                "cargo fmt --check",
                "cargo clippy -- -D warnings"
            ]
        },

        {
            name: "create-release",
            image: "proget.makedeb.org/docker/makedeb/makedeb:ubuntu-jammy",
	        environment: {
		        github_api_key: {from_secret: "github_api_key"}
	        },
            depends_on: ["run-tests"],
            commands: [".drone/scripts/create-release.sh"]
        },

        {
            name: "publish-mpr",
            image: "proget.makedeb.org/docker/makedeb/makedeb:ubuntu-jammy",
	        environment: {
	            ssh_key: {from_secret: "ssh_key"}
	        },
            depends_on: ["create-release"],
            commands: [".drone/scripts/publish-mpr.sh"]
        },

        {
            name: "publish-crates-io",
            image: "proget.makedeb.org/docker/makedeb/makedeb:ubuntu-jammy",
	        environment: {
	            CARGO_REGISTRY_TOKEN: {from_secret: "crates_api_key"}
	        },
            depends_on: ["create-release"],
            commands: [
                "rm makedeb/shoot -rf",
                ".drone/scripts/setup-pbmpr.sh",
                "sudo apt install cargo -y",
                "cargo publish"
            ]
        }
    ]
};

[deploy()]
