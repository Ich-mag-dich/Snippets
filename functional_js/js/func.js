function _filter(list, predi) {
  let new_list = [];
  _each(list, function (val) {
    if (predi(val)) new_list.push(val);
  });
  return new_list;
}

function _map(list, mapper) {
  let new_list = [];
  _each(list, function (val) {
    new_list.push(mapper(val));
  });
  return new_list;
}

function _each(list, iter) {
  for (let i = 0; i < list.length; i++) {
    iter(list[i]);
  }
  return list;
}
function _curry(fn) {
  return function (a, b) {
    return arguments.length == 2
      ? fn(a, b)
      : function (b) {
          return fn(a, b);
        };
  };
}
function _curryr(fn) {
  return function (a, b) {
    return arguments.length == 2
      ? fn(a, b)
      : function (b) {
          return fn(b, a);
        };
  };
}
let _get = _curryr(function (obj, key) {
  return obj == null ? undefined : obj[key];
});

var _map = _curryr(_map),
  _filter = _curryr(_filter);
function _rest(list, num) {
  return slice.call(list, num || 1);
}
function _reduce(list, iter, memo) {
  if (arguments.length == 2) {
    memo = list[0];
    list = _rest(list);
  }
  _each(list, (val) => {
    memo = iter(memo, val);
  });
  return memo;
}

function _pipe() {
  let fns = arguments;
  return function (arg) {
    return _reduce(
      fns,
      (arg, fn) => {
        return fn(arg);
      },
      arg
    );
  };
}

function _go(arg) {
  let fns = _rest(arguments);
  return _pipe.apply(null, fns)(arg);
}
