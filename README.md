# NULLIFY Docker Introduction Presentation
by Matthew Roland

You will need Rust installed to be able to run this program. Not that it does much now, but it will some day. You just need to believe

Yeah... So don't use this to learn Rust or how good programs work. It just works (maybe), and that's good enough for me for rn

This creates a directory at ./data

# Using this Commit with Docker
## Create standard tic-tac-toe client:
```bash
docker build -t ttt-base .
# Tells docker to build
# Tag the resulting container as tttc
# And search . for 'Dockerfile'

docker run -i --rm --name ttt-client ttt-base
# Run a container
# In interactive mode, (if you don't it will infinite loop)
# Remove once it's stopped
# Name it 'ttt-client' instead of it's usual random naming schemes
# Use the 
```

## Create tic-tac-toe Server
```bash
docker build -t ttt-base
docker run --rm --name ttt-server ttt-base tic-tac-toe 
```
