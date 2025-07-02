default:
    @just --list

clean:
    rm -fr container
    mkdir -p pacts
    (cd pacts && rm -fr *)
    (cd proto && rm -fr generated && mkdir -p generated/rust && cd rust_generator && cargo clean)
    (cd consumer && cargo clean)        

consumer-docker:
    #!/bin/bash
    (
        trap "rm -fr container" EXIT
        (cd proto && rm -fr generated && mkdir -p generated/rust && cd rust_generator && cargo clean)
        (cd consumer && cargo clean)        
        rsync -zrLptgoD . container
        mkdir -pv ./container/
        docker build --no-cache --tag="consumer" --file=Dockerfile
    )

generate-pact:
    mkdir -p pacts
    (cd pacts && rm -fr *)
    docker run --mount type=bind,src=./pacts,dst=/usr/src/pacts --replace --name consumer consumer

regenerate-pact: consumer-docker generate-pact