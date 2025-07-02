This project uses the just command runner for convenience.  If you're not familiar with just or don't want to install it, here's the quick start:

    mkdir -p pacts
    (cd pacts && rm -fr *)
    (
        trap "rm -fr container" EXIT
        (cd proto && rm -fr generated && mkdir -p generated/rust && cd rust_generator && cargo clean)
        (cd consumer && cargo clean)        
        rsync -zrLptgoD . container
        mkdir -pv ./container/
        docker build --no-cache --tag="consumer" --file=Dockerfile
    )
    docker run --mount type=bind,src=./pacts,dst=/usr/src/pacts --replace --name consumer consumer
