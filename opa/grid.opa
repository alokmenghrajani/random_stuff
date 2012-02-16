/**
 * Quickly hacked together 2d grid for opa.
 *
 */

type grid('a) =
  {
    int size_x,
    int size_y,
    intmap(intmap('a)) data,   // todo: specify a type for intmap(intmap('a)) ?
  }

// todo: should not be visible outside the module
and outer_for('a) =
  {
    int i,
    intmap(intmap('a)) data,
  }

// todo: same here...
and inner_for('a) =
  {
    int j,
    intmap('a) data,
  }

module Grid {
  /**
   * Initializes a grid of size size_x by size_y.
   *
   * All the elements are set to el.
   */
  function grid('a) initialize(int size_x, int size_y, 'a el) {
    Grid.fill(
      {
        size_x: size_x,
        size_y: size_y,
        data: IntMap.empty,
      },
      el
    )
  }

  /**
   * Todo: probably should not be exposed outside the module...
   */
  function grid('a) fill(grid('a) grid, 'a el) {
    outer_for('a) t = for(
      {i:grid.size_x, data:IntMap.empty},
      function(outer_for('a) s1) {
        inner_for('a) t2 = for(
          {j:grid.size_y, data:IntMap.empty},
          function(inner_for('a) s2) {
            {j:s2.j-1, data:IntMap.add(s2.j-1, el, s2.data)}
          },
          function(inner_for('a) s2) { s2.j > 0}
        )
        {i:s1.i-1, data:IntMap.add(s1.i-1, t2.data, s1.data)}
      },
      function(outer_for('a) s1) { s1.i > 0 }
    )
    {
      size_x: grid.size_x,
      size_y: grid.size_y,
      data: t.data,
    }
  }

  /**
   * Returns the element at position (x,y).
   */
  function option('a) get(int x, int y, grid('a) grid) {
    if ((x >= 0) && (x <= grid.size_x) && (y >= 0) && (y <= grid.size_y)) {
      inner = IntMap.get(x, grid.data)
      match (inner) {
        case {some:data}:
          IntMap.get(y, data)
        case {none}:
          {none}
      }
    } else {
      {none}
    }
  }

  /**
   * Sets the element at position (x, y).
   */
  function grid('a) set(int x, int y, 'a v, grid('a) grid) {
    if ((x >= 0) && (x <= grid.size_x) && (y >= 0) && (y <= grid.size_y)) {
      intmap(intmap('a)) data = IntMap.mapi(
        function(int i, intmap('a) inner_map) {
          if (i != x)
            inner_map
          else
            IntMap.add(y, v, inner_map)
        },
        grid.data
      )
      {
        size_x: grid.size_x,
        size_y: grid.size_y,
        data: data,
      }
    } else
      grid
  }

  /**
   * Mapi over every element in the grid.
   *
   * f is a function that takes i, j, 'a and returns 'b
   *
   * todo: how can I type that?
   */
  function grid('b) mapi(f, grid('a) grid) {
    data = IntMap.mapi(
      function(int x, intmap('a) inner_map) {
        IntMap.mapi(
          function(int y, 'a v) {
            f(x, y, v)
          },
          inner_map
        )
      },
      grid.data
    )
    {
      size_x: grid.size_x,
      size_y: grid.size_y,
      data: data,
    }
  }

  /**
   * Map over every element in the grid.
   *
   * f is a function that takes 'a and returns 'b
   *
   * todo: how can I type that?
   */
  function grid('b) map(f, grid('a) grid) {
    Grid.mapi(function(_, _, 'a el){f(el)}, grid)
  }

  /**
   * Mapi over a subset of the grid.
   *
   * Calls f from (min_x, min_y) to (max_x, max_y).
   *
   * f is a function that takes i, j, 'a and returns 'a
   *
   * todo: how can I type that?
   */
  function grid('a) mapi_range(f, grid('a) grid, int min_x, int max_x, int min_y, int max_y) {
    data = IntMap.mapi(
      function(int x, intmap('a) inner_map) {
        IntMap.mapi(
          function(int y, 'a v) {
            if ((x >= min_x) && (x <= max_x) && (y >= min_y) && (y <= max_y)) {
              f(x, y, v)
            } else {
              v
            }
          },
          inner_map
        )
      },
      grid.data
    )
    {
      size_x: grid.size_x,
      size_y: grid.size_y,
      data: data,
    }
  }
}

type intgrid = grid(int)

module IntGrid {
  function intgrid create(int size_x, int size_y) {
    Grid.create(int size_x, int size_y, 0)
  }
}
