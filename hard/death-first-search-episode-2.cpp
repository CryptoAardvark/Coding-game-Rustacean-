#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <map>
#include <queue>
#include<bits/stdc++.h>

#define NODE_NOT_FOUND      (-1)
#define INF 0x3f3f3f3f

using namespace std;

typedef pair<int, int> iPair;

class Edge {
public:
    Edge(int _v, int _u, int _w = 1) : v(_v), u(_u), w(_w) { served = true; }
    bool exitEdge(int e) { 
        if (v == e || u == e) {
            served = false;
            return true;
        }
        return false;
    }
    void setWeight(int _w) { w = _w;        }
    void setServed()       { served = true; }
    
    int    getFrom() const { return v;      }
    int      getTo() const { return u;      }
    int  getWeight() const { return w;      }
    bool getServed() const { return served; }
    
   
    bool operator!=(const Edge &e) { return !(v == e.v && u == e.u); }
    
    friend bool operator==(const Edge &current, const Edge &data);
    friend bool operator<(const Edge &e1, const Edge &e2);
    friend ostream& operator<<(ostream &stream, const Edge &e);
    friend Edge verifyEdge(const Edge &current, const Edge &data);
    
    int v, u, w;
    bool served;
};
Edge noneEdge(-1, -1, -1);
bool operator<(const Edge &e1, const Edge &e2) {
    if (e1.v < e2.v) return true;
    else if (e1.v == e2.v && e1.u < e2.u) return true;
    else return false;
}
ostream& operator<<(ostream &stream, const Edge &e) {
    stream << e.v << " --> " << e.u << "  s| " << e.served << "  w| " << e.w;
    return stream;
}
bool operator==(const Edge &current, const Edge &data) {
    return (current.v == data.v && current.u == data.u); 
}
Edge verifyEdge(const Edge &current, const Edge &data) {
    if (current == data) return data;
    else return noneEdge;
}

class Graph
{
    int V; 
    list< pair<int, int> > *adj;
 
public:
    Graph(int V);
    void addEdge(int v, int u, int w);

    int shortestPath(int s, vector<int> &f);
};
 
Graph::Graph(int V) {
    this->V = V;
    adj = new list<iPair> [V];
}
void Graph::addEdge(int v, int u, int w) {
    adj[v].push_back(make_pair(u, w));
    adj[u].push_back(make_pair(v, w));
}
int Graph::shortestPath(int src, vector<int> &first_target) {
    
    priority_queue< iPair, vector <iPair> , greater<iPair> > pq;
    vector<int> dist(V, INF);
    pq.push(make_pair(0, src));
    dist[src] = 0;
 
    while (!pq.empty())
    {
        int u = pq.top().second;
        pq.pop();
        list< pair<int, int> >::iterator i;
        
        for (i = adj[u].begin(); i != adj[u].end(); ++i) {
            int v = (*i).first;
            int weight = (*i).second;
 
            if (dist[v] > dist[u] + weight) {
                dist[v] = dist[u] + weight;
                pq.push(make_pair(dist[v], v));
            }
        }
    }
 
    //cerr << "Vertex   Distance from Source\n";
    map<int, int> to_print;
    for (int i = 0; i < V; ++i) {
        //cerr << i << " \t " << dist[i] << endl;
        for (int j = 0; j < first_target.size(); ++j) 
            to_print.insert(pair<int, int>(dist[first_target[j]], first_target[j]));
        
    }
    auto it = to_print.begin();
    return it->second;
}
 

int main()
{
    int N; // the total number of nodes in the level, including the gateways
    int L; // the number of links
    int E; // the number of exit gateways
    cin >> N >> L >> E; cin.ignore();
    
    vector<Edge> edge;
    vector<int> Vertex, data_exit;

    for (int i = 0; i < L; i++) {
        int N1, N2; // N1 and N2 defines a link between these nodes
        cin >> N1 >> N2; cin.ignore();
        
        edge.push_back(Edge(N1, N2));
        edge.push_back(Edge(N2, N1));
        Vertex.push_back(N2);
        Vertex.push_back(N1);
    }
    sort(edge.begin(), edge.end());
    sort(Vertex.begin(), Vertex.end());
    auto it = unique(Vertex.begin(), Vertex.end());
    Vertex.resize(distance(Vertex.begin(), it));
    
    int V = Vertex.size();
    
    for (int i = 0; i < E; i++) {
        int EI; // the index of a gateway node
        cin >> EI; cin.ignore();
        
        data_exit.push_back(EI);
        for (auto it = edge.begin(); it != edge.end(); ++it) {
            it->exitEdge(EI);
        }
        Vertex.erase(find(Vertex.begin(), Vertex.end(), EI));
    }
    
    vector<int> first_target, for_weight;
    for (int i = 0; i < Vertex.size(); i++) {
        int count = 0;
        for (auto it = edge.begin(); it != edge.end(); ++it) {
            if (it->v == Vertex[i]) {
                for (int j = 0; j < data_exit.size(); ++ j) {
                    if (it->u == data_exit[j]) {
                        ++count;
                        for_weight.push_back(Vertex[i]);
                    }
                }
                
                if (count > 1) first_target.push_back(Vertex[i]);
            }
        }
    }
    
    sort(for_weight.begin(), for_weight.end());
    auto iw = unique(for_weight.begin(), for_weight.end());
    for_weight.resize(distance(for_weight.begin(), iw));
    
    sort(first_target.begin(), first_target.end());
    auto ip = unique(first_target.begin(), first_target.end());
    first_target.resize(distance(first_target.begin(), ip));
    
    for (int i = 0; i < edge.size(); i++) {
        for (int j = 0; j < for_weight.size(); j++) {
            if (edge[i].v == for_weight[j]) {
                for (int k = 0; k < for_weight.size(); k++) {
                    if (edge[i].u == for_weight[k]) edge[i].setWeight(0);
                }
            }
        }
    }
    
    Graph graph(V);
    for(auto &it: edge) {
        //cerr << it << endl;
        graph.addEdge(it.getFrom(), it.getTo(), it.getWeight());
    }
    
    while (1) {
        int SI; // The index of the node on which the Skynet agent is positioned this turn
        cin >> SI; cin.ignore();
        
        bool find_edge = false;
        for (int i = 0; i < data_exit.size(); ++i) {
            Edge current(SI, data_exit[i]);
            for (int j = 0; j < edge.size(); ++j) {
                if (verifyEdge(current, edge[j]) != noneEdge && !edge[j].getServed()) {
                    cout << edge[j].getFrom() << " " << edge[j].getTo() << endl;
                    edge[j].setServed();
                    auto it = find(edge.begin(), edge.end(), Edge(data_exit[i], SI));
                    it->setServed();
                    find_edge = true;
                    break;
                }
            }
            if (find_edge) break;
        }
    
        if (find_edge) continue;
        else if (!find_edge && !first_target.empty()) {
            int print_vertex = graph.shortestPath(SI, first_target);
        
            for (int i = 0; i < data_exit.size(); ++i) {
                Edge current(print_vertex, data_exit[i]);
                bool find_current = false;
                for (int j = 0; j < edge.size(); ++j) {
                    if (verifyEdge(current, edge[j]) != noneEdge && !edge[j].getServed()) {
                        cout << edge[j].getFrom() << " " << edge[j].getTo() << endl;
                        edge[j].setServed();
                        auto it = find(edge.begin(), edge.end(), Edge(data_exit[i], print_vertex));
                        it->setServed();
                        first_target.erase(find(first_target.begin(), first_target.end(), print_vertex));
                        find_current = true;
                        break;
                    }
                }
                if (find_current) break;
            }
        }
        else {
            for (int i = 0; i < edge.size(); ++i) {
                if(!edge[i].getServed()) {
                    cout << edge[i].getFrom() << " " << edge[i].getTo() << endl;
                    edge[i].setServed();
                    auto it = find(edge.begin(), edge.end(), Edge(edge[i].getTo(), edge[i].getFrom()));
                    it->setServed();
                    break;
                }
                
            }
        }
    }
    
}