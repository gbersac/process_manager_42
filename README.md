# process_manager_42
Project manager. Each project is a collection of resource and processes which consume/produce those resources. This program try to optimize the production of some resources.

The problem can be depicted as a [timed petri net](https://en.wikipedia.org/wiki/Petri_net) (see petri.pdf in french).

## Properties
Properties of this network :
- time: when a process is triggered, it consumes its prerequisites imediately, but produce its creations within a delay
- infinite instances of processes : each process can be triggered an infinite time as long as all its prerequisites are fulfielled.
