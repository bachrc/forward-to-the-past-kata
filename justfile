[private]
default:
    just --list

# lance les tests du projet
test:
    cargo nextest run --cargo-quiet

# construit le binaire de sortie
build:
    cargo build --release