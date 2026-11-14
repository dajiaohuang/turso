import { expect, test } from 'vitest';
import { bindParams } from './bind.js';

function statement(names: (string | null)[]) {
  const values = new Map<number, unknown>();
  return {
    values,
    parameterCount: () => names.length,
    parameterName: (index: number) => names[index - 1],
    bindAt: (index: number, value: unknown) => values.set(index, value),
  };
}

test('missing positional and named values fail without changing existing bindings', () => {
  const positional = statement([null, null]);
  bindParams(positional, ['first', 'second']);
  expect(() => bindParams(positional, ['third'])).toThrow('Too few parameter values');
  expect([...positional.values.values()]).toEqual(['first', 'second']);
  expect(() => bindParams(positional, [])).toThrow('Too few parameter values');

  const named = statement([':key', ':value']);
  bindParams(named, [{ key: 'first', value: 'second' }]);
  expect(() => bindParams(named, [{ key: 'third' }])).toThrow('Too few parameter values');
  expect([...named.values.values()]).toEqual(['first', 'second']);
  bindParams(named, [{ key: null, value: 'third' }]);
  expect([...named.values.values()]).toEqual([null, 'third']);
});

test('a statement without placeholders needs no arguments', () => {
  expect(() => bindParams(statement([]), [])).not.toThrow();
});
