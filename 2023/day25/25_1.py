import os
import networkx as nx

script_directory = os.path.dirname(os.path.abspath(__file__))
file_path = os.path.join(script_directory, '25_input.txt')
input_file = open(file_path, 'r')

G = nx.Graph()
for line in input_file:
    line = line.strip()
    name, connected = line.split(':')
    connected = connected.split()
    G.add_node(name)
    for comp in connected:
        if not G.has_node(comp):
            G.add_node(comp)
        G.add_edge(name, comp)
    
cut = nx.minimum_edge_cut(G)
G.remove_edges_from(cut)

groups = nx.connected_components(G)

res = 1
for group in groups:
    res *= len(group)

print(res)