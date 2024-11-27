#include <array>
#include <assert.h>
#include <cstdint>
#include <iostream>
#include <map>
#include <queue>
#include <set>
#include <string>
#include <tuple>
#include <utility>
#include <vector>

#include "debug.cpp"

using i32 = int32_t;
using u32 = uint32_t;

// Four adjascent and no move
const i32 DX[5] = {0, 0, -1, 1, 0};
const i32 DY[5] = {-1, 1, 0, 0, 0};

struct Piece {
  bool u, r, d, l, m;
  Piece(bool u = false, bool r = false, bool d = false, bool l = false,
        bool m = true)
      : u(u), r(r), d(d), l(l), m(m) {}

  bool operator<(const Piece &o) const {
    return std::array<bool, 5>{u, r, d, l, m} <
           std::array<bool, 5>{o.u, o.r, o.d, o.l, o.m};
  }

  void rotateClockwise(void) {
    auto newPiece = *this;
    newPiece.u = l;
    newPiece.r = u;
    newPiece.d = r;
    newPiece.l = d;
    *this = newPiece;
  }

  friend std::ostream &operator<<(std::ostream &target, const Piece &source) {
    target << "(" << source.u << ", " << source.r << ", " << source.d << ", "
           << source.l << ")";
    return target;
  }
};

/*
 * ARRUMA ESSA BOMBA AQUI DEPOIS !
 * */
bool canConnect(const Piece &p1, i32 x1, i32 y1, const Piece &p2, i32 x2,
                i32 y2) {
  if (x1 == x2 and y1 == y2)
    return true;

  int dx = x1 - x2, dy = y1 - y2;

  assert(!dx or !dy); // se não tá diagonal

  if (dx) {
    if (dx > 0) { // está a cima
      return p1.u and p2.d;
    } else { // está a baixo
      return p1.d and p2.u;
    }
  } else {
    if (dy > 0) { // está a esquerda
      return p1.l and p2.r;
    } else { // restar a direita
      return p1.r and p2.l;
    }
  }
  return false;
}

const Piece PIECES[6] = {{0, 1, 0, 1}, {1, 0, 1, 0}, {0, 0, 1, 1},
                         {1, 0, 0, 1}, {1, 1, 0, 0}, {0, 1, 1, 0}};

using GameGrid = std::vector<std::vector<Piece>>;

// Global rocks !
i32 numberOfRows, numberOfColumns;
i32 startX, startY;
i32 endX, endY;

bool acessibleCoordinate(i32 x, i32 y) {
  return x >= 0 and x < numberOfRows and y >= 0 and y < numberOfColumns;
}

struct GameState {
  i32 x, y;
  GameGrid grid;

  GameState(int x, int y, const GameGrid &grid) : x(x), y(y), grid(grid){};

  GameState(int n = numberOfRows, int m = numberOfColumns, int x = startX,
            int y = startY)
      : x(x), y(y), grid(n, std::vector<Piece>(m)) {}

  inline i32 manhattanDistanceToGoal() const {
    return (abs(x - endX) + abs(y - endY));
  }

  bool operator<(const GameState &other) const {
    using T = tuple<i32, i32, GameGrid>;
    return T{x, y, grid} < T{other.x, other.y, other.grid};
  }

  inline bool reachedGoal(void) const { return x == endX and y == endY; }

  bool reachableFromStart() {
    std::vector vis(numberOfRows, std::vector<bool>(numberOfColumns, 0));
    std::queue<std::pair<i32, i32>> q;

    vis[startX][startY] = true;
    q.emplace(startX, startY);

    while (!q.empty()) {
      auto [ux, uy] = q.front();
      q.pop();

      if (ux == x and uy == y) {
        return true;
      }

      for (i32 d = 0; d < 4; d++) {
        i32 x2 = ux + DX[d];
        i32 y2 = uy + DY[d];

        if (!acessibleCoordinate(x2, y2))
          continue;

        if (vis[x2][y2])
          continue;

        if (canConnect(grid[ux][uy], ux, uy, grid[x2][y2], x2, y2)) {
          vis[x2][y2] = true;
          q.emplace(x2, y2);
        }
      }
    }

    return false;
  }
};

using pqState = std::tuple<i32, GameState>;

GameState readInput(void) {
  std::cin >> numberOfRows >> numberOfColumns;
  std::cin >> startX >> startY, startX--, startY--;
  std::cin >> endX >> endY, endX--, endY--;
  GameState ret;
  for (i32 i = 0; i < numberOfRows; i++) {
    std::string s;
    std::cin >> s;
    for (i32 j = 0; j < numberOfColumns; j++) {
      ret.grid[i][j] = PIECES[s[j] - '0'];
    }
  }
  ret.grid[startX][startY].m = ret.grid[endX][endY].m = false;
  return ret;
}

void print_grid(const GameState &g) {
  for (int i = 0; i < numberOfRows; i++) {
    for (int j = 0; j < numberOfColumns; j++) {
      if (g.grid[i][j].l and g.grid[i][j].r) {
        std::cout << "═";
      } else if (g.grid[i][j].u and g.grid[i][j].d) {
        std::cout << "║";
      } else if (g.grid[i][j].l and g.grid[i][j].d) {
        std::cout << "╗";
      } else if (g.grid[i][j].l and g.grid[i][j].u) {
        std::cout << "╝";
      } else if (g.grid[i][j].u and g.grid[i][j].r) {
        std::cout << "╚";
      } else if (g.grid[i][j].r and g.grid[i][j].d) {
        std::cout << "╔";
      }
    }
    std::cout << "\n";
  }
}

i32 main() {

  auto initialGrid = readInput();

  std::priority_queue<pqState, vector<pqState>, greater<pqState>> pq;

  pq.emplace(pqState{initialGrid.manhattanDistanceToGoal(), initialGrid});
  std::set<GameState> vis; // Slow as f*** make it an unordered_set later
  std::map<GameState, GameState>
      father; // Slow as f*** make it an unordered_set later

  vis.emplace(initialGrid);

  auto recover_path = [&](GameState u) {
    vector<GameState> vec{u};

    while (father.count(u)) {
      u = father[u];
      vec.emplace_back(u);
    }
  };

  while (!pq.empty()) {
    auto [uDist, uGameState] = pq.top();
    pq.pop();

    std::cout << '\n';
    print_grid(uGameState);
    std::cout << '\n';

    if (uGameState.reachedGoal()) {
      std::cout << "Killed it !" << std::endl;
      print_grid(uGameState);
      return 0;
    }

    for (u32 d = 0; d < 5; d++) {
      i32 x2 = uGameState.x + DX[d];
      i32 y2 = uGameState.y + DY[d];

      if (not acessibleCoordinate(x2, y2))
        continue;

      GameState v{x2, y2, uGameState.grid};

      for (u32 r = 0; r < 4; r++) {
        if (v.reachableFromStart() and !vis.count(v)) {

          father[v] = uGameState;
          vis.emplace(v);
          pq.emplace(v.manhattanDistanceToGoal(), v);
        }

        // TA ROTACIONANDO O QUE NÃO PODE !
        // rotacionar o x2,y2
        if (v.grid[x2][y2].m)
          v.grid[x2][y2].rotateClockwise();
      }
    }
  }

  std::cout << "Se torou KKKKK" << std::endl;
}
