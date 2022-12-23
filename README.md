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

