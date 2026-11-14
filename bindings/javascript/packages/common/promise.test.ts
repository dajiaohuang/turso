import { expect, test } from 'vitest'
import { maybePromise, maybeValue, Statement } from './promise.js'

test('drizzle-orm', async () => {
    const lazy = maybePromise(() => fetch('http://google.com'));
    let status, headers;
    //@ts-ignore
    lazy.apply(x => { status = x.status; })
    //@ts-ignore
    lazy.apply(x => { headers = x.headers; })
    let response = await lazy.resolve();
    expect(response).not.toBeNull();
    expect(status).toBe(200);
    expect(headers).not.toBeNull();
})

test('bind applies to eager, lazy, and already resolved statements', async () => {
    function nativeStatement() {
        const values = new Map<number, unknown>();
        return {
            values,
            parameterCount: () => 1,
            parameterName: () => null,
            bindAt: (index: number, value: unknown) => values.set(index, value),
        };
    }
    const eager = nativeStatement();
    const eagerStatement = new Statement(maybeValue(eager as any), {} as any, {} as any, async () => {});
    expect(eagerStatement.bind('first')).toBe(eagerStatement);
    expect(eager.values.get(1)).toBe('first');

    const lazyNative = nativeStatement();
    const lazy = maybePromise(async () => lazyNative as any);
    const lazyStatement = new Statement(lazy, {} as any, {} as any, async () => {});
    expect(lazyStatement.bind('second')).toBe(lazyStatement);
    await lazy.resolve();
    expect(lazyNative.values.get(1)).toBe('second');
    lazyStatement.bind('third');
    expect(lazyNative.values.get(1)).toBe('third');
});

test('a setting applied while a lazy statement is resolving is not lost', async () => {
    let finish: (value: { value: number }) => void;
    const lazy = maybePromise(() => new Promise<{ value: number }>(resolve => { finish = resolve; }));
    const pending = lazy.resolve();
    lazy.apply(value => { value.value = 2; });
    finish({ value: 1 });
    expect((await pending).value).toBe(2);
});

test('settings applied while resolving keep their order', async () => {
    let finish: (value: { value: number }) => void;
    const lazy = maybePromise(() => new Promise<{ value: number }>(resolve => { finish = resolve; }));
    const pending = lazy.resolve();
    lazy.apply(value => { value.value = 2; });
    finish({ value: 1 });
    await Promise.resolve();
    lazy.apply(value => { value.value = 3; });
    expect((await pending).value).toBe(3);
});
