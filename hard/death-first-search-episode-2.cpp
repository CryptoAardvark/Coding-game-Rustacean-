#include <iostream>
#include <string>
#include <vector>
#include <algorithm>
#include <set>
#include <queue>
#include <stdio.h>

using namespace std;
struct Node {
  set<int> links;
  bool gateways;
  set<int> tGatewayLinks;
};

pair<int, int> bfs(vector<Node>& graph, int s) {
  vector<bool> visited(graph.size(), false);
  queue<int> Q;
  Q.push(s);
  visited[s] = true;

  int selectedNode = -1;

  while(!Q.empty()){
    int id = Q.front();
    Q.pop();
    Node n = graph[id];
    visited[id] = true;

    const auto push_neighbours = [&n, &Q, &visited]() {
      for (const int& nid : n.links) {
        if (not visited[nid]) Q.push(nid);
      }
    };

    if(n.tGatewayLinks.size() > 1) {
      selectedNode = id;
      break;
    }else if (n.tGatewayLinks.size() == 1) {
      if(selectedNode == -1) {
        selectedNode = id;
        cerr << "selectedNode: " << id;
        if (id == s) break;
      } 
      push_neighbours();
    }
    else if (selectedNode == -1) push_neighbours();

  }

  return make_pair(selectedNode, *(graph[selectedNode].tGatewayLinks.begin()));

}


int main() {
  int N;
  int L;
  int E;

  scanf("%d%d%d", &N, &L, &E);

  vector<Node> graph(N, {set<int>(), false, set<int>()});

  for (int i = 0; i < L; i++){
    int N1, N2;
    scanf("%d%d", &N1, &N2);
    graph[N1].links.insert(N2);
    graph[N2].links.insert(N1);
  }

  for (int i = 0; i < E; i++) {
    int EI;
    scanf("%d", &EI);
    graph[EI].gateways = true;
    for (const int& nid: graph[EI].links) graph[nid].tGatewayLinks.insert(EI);
  }

  while(true) {
    int st;
    scanf("%d", &st);

    pair<int, int> link = bfs(graph, st);
    printf("%d %d\n", link.first, link.second);
    graph[link.first].tGatewayLinks.erase(link.second);
    graph[link.first].links.erase(link.second);
    graph[link.second].links.erase(link.first);

  }

}