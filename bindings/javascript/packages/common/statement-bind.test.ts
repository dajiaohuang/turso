import { expect, test } from 'vitest';
import { Database as PromiseDatabase } from './promise.js';
import { Database as CompatDatabase } from './compat.js';

function nativeDatabase(names: (string | null)[]) {
  const values = new Map<number, unknown>();
  const statement = {
    parameterCount: () => names.length,
    parameterName: (index: number) => names[index - 1],
    bindAt: (index: number, value: unknown) => values.set(index, value),
    setQueryTimeout() {},
    reset() {},
    stepSync: () => [2, 0],
    finalize() {},
  };
  const db = {
    open: true,
    connectSync() {},
    async connectAsync() {},
    prepare: () => statement,
  };
  return { db: db as any, values };
}

test('promise statements reject omitted values and retain explicit bindings', async () => {
  const native = nativeDatabase([null]);
  const db = new PromiseDatabase(native.db);
  const statement = await db.prepare('SELECT ?');
  await statement.get('first');
  await expect(statement.get()).rejects.toThrow('Too few parameter values');
  expect(native.values.get(1)).toBe('first');
  expect(statement.bind('permanent')).toBe(statement);
  await statement.get();
  await statement.get();
  expect(native.values.get(1)).toBe('permanent');
});

test('compat statements reject missing named values and retain explicit bindings', () => {
  const native = nativeDatabase([':key', ':value']);
  const db = new CompatDatabase(native.db);
  const statement = db.prepare('SELECT :key, :value');
  statement.get({ key: 'first', value: 'second' });
  expect(() => statement.get({ key: 'third' })).toThrow('Too few parameter values');
  expect([...native.values.values()]).toEqual(['first', 'second']);
  expect(statement.bind({ key: 'bound', value: null })).toBe(statement);
  statement.get();
  statement.get();
  expect([...native.values.values()]).toEqual(['bound', null]);
});
