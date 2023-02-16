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

## Iteration One
  
  - No mesh network
    - meaning you can not see nodes through other nodes
  - This will just be the transportation layer
    - meaning that you will just be given a list of valid peers to talk to
    - The pub/sub interface will implimented on top of this layer
    - Data is shapeless at this layer
  - API
    - Recv channel
      - data recieved will just be raw bytes
    - Will be able to query peers in network
      - amount
      - IPs
      - status (active or not)
    - choose a peer on the network to send data to
    - Heartbeat
      - broad casting will also act as a heartbeat
      - peers will be updated based on thier heartbeat
    

