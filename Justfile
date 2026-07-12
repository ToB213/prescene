default:
    cargo run -- build examples/basic.yaml \
    --output output/index.html
custom:
    cargo run -- build examples/basic.yaml \
    --css examples/custom.css \
    --output output/index.html
