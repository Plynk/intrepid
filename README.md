# Intrepid 
### Pub Sub Mesh Network

## Design goals

  - Subscriber designated publishing rates
    - minimum publishing rate
    - desired publishing rate
  - choice between TCP and UDP
    - some messages will be able to be dropped
    - some messages need a garentee 
      - start with only TCP?
  - Optimized packet routing based on traffic
  - packet priorities
    - allow higher priority packets to not be dropped
  - CLI
    - observe devices on the network and the topics they are publishing
    - observe what devices each devices wants


## Node 
  - device type?
  - uuid
  - pub topics
  - sub topics
  - network peers?
  - ip 

## Intrepid socket
  - Communication
    - all data is bytes
    - listening thread
    - sending thread
  - Discovery 
    - broadcast to peers
    - listen to all peers

## TODO
### general
  - Data size
    - fragmentation?
    - fix sill broadcast size const
  - what is a peer?
  - routing
  - make destination general for intrepid node trait sending function
### broadcast discovery 
  - initially going to directly compare stored nodes to broadcasted nodes,
    - may be worth in the future adding a hash to make checking for updates cheaper
  - timestamping broadcasts
    - nodes need to be heartbeat their broadcast or else they are dropped

