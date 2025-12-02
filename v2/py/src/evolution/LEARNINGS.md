965da26d - 2 control neuron system cannot handle bad food avoidance.
2fe75505 - seed 457 - sample0.json - Demonstrated evolving bad food eating response
continuous kill during sim better than kill at end
tune both kill rate (start low go higher) and evolution rate (start high go lower) as progress made on metrics
    If organisms dying out completely -> lower kill rate
    If organism numbers fluctuating but metric not moving -> lower evolution rate
    Population explodes -> increase kill rate
    A low evolution rate does not hold back the population growth rate (under a given set of conditions), so if one the the previous two is not in place, the population should explode
    Adjust learning rate on both nodes and edges
    Bias towards fewer edges (kill / remove nodes/edges during reproduction)
    Chance for duplicate child organisms
    Increase brain processing step count
b224ccf - Sample 200 - Highest acheived int kill rate 0.0925