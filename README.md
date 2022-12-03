# Evolution sim

Idea: Evolve strategies for solving particular problems

Potential problems:
    Regression task
    Image classification

Next steps:
    Add a learning circuit
        Overall neural architecture:
        s = stim reception
        a = stim response (action)
        n = selection only response
        l = learned response
        u = (under selection) update circuit
        r = reward/death (can treat this as the same function for now, but should really be 
                          different, but highly correlated)
            
                 OUTPUT
               a   u   l   r
        =====================
          s ||   | x |   |   |
        - - - - - - - - - - -
          n || x |   |   |   |
        - - - - - - - - - - -
INPUT     u ||   |   | x |   |
        - - - - - - - - - - -
          l || x |   |   |   |
        - - - - - - - - - - -
          r ||   | x |   |   |
        - - - - - - - - - - -
          a ||   |   |   | x |

        Learning
            {s -> (n, l) -> a -> r} runs
            {(s, r) -> u} runs
            output of u assigned to l
            (Repeat m times)?, then go through Selection
        Selection
            {s -> (n, l) -> a -> r} runs
            Organism lives or dies based on r
            Return to Learning

        All neural circuits are under selection. 
        Additionally, the value of l is assigned during learning by the learning circuit

Future Ideas:
    Structure NN connections based on genes
        Instead of single neurons/connections, may want the ability for genes to specify entire
        common structures (including learning, IO, etc) with a small number of genes
    Sexual selection
        Place sexual metrics under natural selection as well
