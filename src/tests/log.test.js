/**
    Tests > Log

    tests the functionality of the app Log class located in ./src/classes/Log.js

    Log.js reads NODE_ENV and process.type once when it is imported, so tests that
    need a different environment reload the module with loadLog().

    @docs               https://vitest.dev/api/expect
*/

import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';

/*
    shared mocks; reset before every test because initializeLogSystem() replaces
    electron-log methods with no-ops when NODE_ENV=test
*/

const mocks = vi.hoisted( () => (
{
    log: {},
    app: {},
    getAllWindows: null,
    existsSync: null,
    mkdirSync: null
}) );

vi.mock( 'fs', async( importOriginal ) =>
{
    const actual = await importOriginal();
    const fs =
    {
        ...actual,
        existsSync: ( ...args ) => mocks.existsSync( ...args ),
        mkdirSync: ( ...args ) => mocks.mkdirSync( ...args )
    };

    return { ...fs, default: fs };
});

vi.mock( 'electron-log', () => ({ default: mocks.log }) );

vi.mock( 'electron', () =>
{
    const electron =
    {
        app: mocks.app,
        BrowserWindow:
        {
            getAllWindows: ( ...args ) => mocks.getAllWindows( ...args )
        }
    };

    return { ...electron, default: electron };
});

const resetLogMock = () =>
{
    for ( const key of Object.keys( mocks.log ) )
        delete mocks.log[ key ];

    Object.assign( mocks.log,
    {
        initialize: vi.fn(),
        error: vi.fn(),
        warn: vi.fn(),
        info: vi.fn(),
        debug: vi.fn(),
        verbose: vi.fn(),
        transports:
        {
            file: { level: 'info' },
            console: { level: 'info' },
            ipc: { level: 'info' },
            remote: { level: 'info' }
        }
    });

    mocks.app.getAppPath = vi.fn( () => '/tmp/ntfy-desktop-log-test' );
    mocks.getAllWindows = vi.fn( () => [] );
    mocks.existsSync = vi.fn( () => true );
    mocks.mkdirSync = vi.fn();
};

/*
    re-import Log.js with the given environment
*/

const loadLog = async({ env = {}, processType } = {}) =>
{
    for ( const [ key, value ] of Object.entries( env ) )
    {
        if ( value === undefined )
            delete process.env[ key ];
        else
            process.env[ key ] = value;
    }

    if ( processType === undefined )
        delete process.type;
    else
        process.type = processType;

    vi.resetModules();
    return import( '#log' );
};

const flush = () => new Promise( ( resolve ) => setTimeout( resolve, 0 ) );
const waitForInitTimer = () => new Promise( ( resolve ) => setTimeout( resolve, 150 ) );   // Log.js logs 100ms after init

describe( 'Log Class Tests', () =>
{
    let savedEnv;
    let consoleSpies;

    beforeEach( () =>
    {
        savedEnv = { ...process.env };
        resetLogMock();

        consoleSpies = {};
        for ( const method of [ 'log', 'info', 'warn', 'error', 'debug', 'trace' ] )
            consoleSpies[ method ] = vi.spyOn( console, method ).mockImplementation( () => {});
    });

    afterEach( () =>
    {
        for ( const key of Object.keys( process.env ) )
        {
            if ( !( key in savedEnv ) )
                delete process.env[ key ];
        }

        Object.assign( process.env, savedEnv );
        delete process.type;
        vi.restoreAllMocks();
    });

    /*
        Log > message cleaning
    */

    describe( 'Message formatting', () =>
    {
        it( 'should strip ANSI codes and stringify symbols and numbers for file logs', async() =>
        {
            const { default: Log } = await loadLog();

            Log.info( '\x1b[31mred\x1b[0m', Symbol( 'sym' ), 42 );

            expect( mocks.log.info ).toHaveBeenCalledWith( '[info]', 'red Symbol(sym) 42' );
            expect( consoleSpies.info ).toHaveBeenCalled();
            expect( consoleSpies.info.mock.calls[ 0 ][ 3 ] ).toContain( 'Symbol(sym) 42' );
        });

        it( 'should prefix console output with a timestamp', async() =>
        {
            const { default: Log } = await loadLog();

            expect( Log.now() ).toMatch( /\[.+\]/ );
        });
    });

    /*
        Log > levels
    */

    describe( 'Log levels', () =>
    {
        const cases = [
            [ 'verbose', 6, 'debug', '[verbose]', 'debug' ],
            [ 'ok', 4, 'info', '[ok]', 'log' ],
            [ 'notice', 3, 'warn', '[notice]', 'log' ],
            [ 'warn', 2, 'warn', '[warn]', 'warn' ],
            [ 'error', 1, 'error', '[error]', 'error' ],
            [ 'info', 4, 'info', '[info]', 'info' ]
        ];

        it.each( cases )( 'Log.%s should log at LOG_LEVEL=%i', async( method, level, logFn, prefix, consoleFn ) =>
        {
            const { default: Log } = await loadLog({ env: { LOG_LEVEL: String( level ) } });

            Log[ method ]( 'hello', 'world' );

            expect( mocks.log[ logFn ] ).toHaveBeenCalledWith( prefix, 'hello world' );
            expect( consoleSpies[ consoleFn ] ).toHaveBeenCalled();
        });

        it.each( cases )( 'Log.%s should be silent below LOG_LEVEL=%i', async( method, level, logFn ) =>
        {
            const { default: Log } = await loadLog({ env: { LOG_LEVEL: String( level - 1 || -1 ) } });

            Log[ method ]( 'hidden' );

            expect( mocks.log[ logFn ] ).not.toHaveBeenCalled();
        });

        it( 'should default to LOG_LEVEL=4 when unset or invalid', async() =>
        {
            const { default: Log } = await loadLog({ env: { LOG_LEVEL: 'nope' } });

            Log.info( 'shown' );
            Log.debug( 'hidden' );

            expect( mocks.log.info ).toHaveBeenCalled();
            expect( mocks.log.debug ).not.toHaveBeenCalled();
        });

        it( 'Log.debug should use console.debug at LOG_LEVEL=5', async() =>
        {
            const { default: Log } = await loadLog({ env: { LOG_LEVEL: '5' } });

            Log.debug( 'dbg' );

            expect( mocks.log.debug ).toHaveBeenCalledWith( '[debug]', 'dbg' );
            expect( consoleSpies.debug ).toHaveBeenCalled();
            expect( consoleSpies.trace ).not.toHaveBeenCalled();
        });

        it( 'Log.debug should use console.trace at LOG_LEVEL=7', async() =>
        {
            const { default: Log } = await loadLog({ env: { LOG_LEVEL: '7' } });

            Log.debug( 'trace me' );

            expect( mocks.log.debug ).toHaveBeenCalledWith( '[debug]', 'trace me' );
            expect( consoleSpies.trace ).toHaveBeenCalled();
        });

        it( 'Log.info should not throw if electron-log fails', async() =>
        {
            const { default: Log } = await loadLog();
            mocks.log.info.mockImplementation( () =>
            {
                throw new Error( 'disk full' );
            });

            expect( () => Log.info( 'boom' ) ).not.toThrow();
            expect( consoleSpies.warn ).toHaveBeenCalledWith( 'Error in logging:', expect.any( Error ) );
        });
    });

    /*
        Log > development vs production console output
    */

    describe( 'Console output by environment', () =>
    {
        it.each( [
            [ 'development', { NODE_ENV: 'development' }],
            [ 'DEV_MODE', { NODE_ENV: 'staging', DEV_MODE: 'true' }],
            [ 'unset NODE_ENV', { NODE_ENV: undefined }]
        ] )( 'should write to the console in %s', async( label, env ) =>
        {
            const { default: Log } = await loadLog({ env: { ...env, LOG_LEVEL: '7' } });
            consoleSpies.trace.mockClear();

            Log.verbose( 'v' );
            Log.debug( 'd' );
            Log.error( 'e' );

            expect( consoleSpies.debug ).toHaveBeenCalled();
            expect( consoleSpies.trace ).toHaveBeenCalled();
            expect( consoleSpies.error ).toHaveBeenCalled();
        });

        it( 'should only write to electron-log in production', async() =>
        {
            const { default: Log } = await loadLog({ env: { NODE_ENV: 'production', DEV_MODE: undefined, LOG_LEVEL: '7' } });
            await flush();
            mocks.log.info = vi.fn();
            mocks.log.warn = vi.fn();
            mocks.log.error = vi.fn();
            mocks.log.debug = vi.fn();
            for ( const spy of Object.values( consoleSpies ) )
                spy.mockClear();

            Log.verbose( 'v' );
            Log.debug( 'd' );
            Log.info( 'i' );
            Log.ok( 'o' );
            Log.notice( 'n' );
            Log.warn( 'w' );
            Log.error( 'e' );

            expect( mocks.log.debug ).toHaveBeenCalledTimes( 2 );
            expect( mocks.log.info ).toHaveBeenCalledTimes( 2 );
            expect( mocks.log.warn ).toHaveBeenCalledTimes( 2 );
            expect( mocks.log.error ).toHaveBeenCalledTimes( 1 );

            for ( const spy of Object.values( consoleSpies ) )
                expect( spy ).not.toHaveBeenCalled();
        });

        it( 'Log.debug at LOG_LEVEL=5 should not write to the console in production', async() =>
        {
            const { default: Log } = await loadLog({ env: { NODE_ENV: 'production', DEV_MODE: undefined, LOG_LEVEL: '5' } });
            await flush();
            consoleSpies.debug.mockClear();

            Log.debug( 'd' );

            expect( consoleSpies.debug ).not.toHaveBeenCalled();
        });
    });

    /*
        Log > broadcast
    */

    describe( 'broadcast()', () =>
    {
        const makeWindow = () => ({ webContents: { executeJavaScript: vi.fn() } });

        it( 'should color the app name when the first argument is true', async() =>
        {
            const { default: Log } = await loadLog();
            const win = makeWindow();

            Log.broadcast( win, true, 'hello', 'there' );

            expect( consoleSpies.log.mock.calls[ 0 ][ 1 ] ).toBe( 'hello there' );
            expect( win.webContents.executeJavaScript.mock.calls[ 0 ][ 0 ] ).toContain( '%c hello there' );
        });

        it( 'should print plain text without a color flag', async() =>
        {
            const { default: Log } = await loadLog();
            const win = makeWindow();

            Log.broadcast( win, 'plain', 'text' );

            expect( consoleSpies.log ).toHaveBeenCalledWith( 'plain text' );
            expect( win.webContents.executeJavaScript ).toHaveBeenCalledWith( 'console.log("plain text")' );
        });

        it( 'should drop a false color flag', async() =>
        {
            const { default: Log } = await loadLog();

            Log.broadcast( makeWindow(), false, 'msg' );

            expect( consoleSpies.log ).toHaveBeenCalledWith( 'msg' );
        });

        it( 'should only log to the console without a window', async() =>
        {
            const { default: Log } = await loadLog();

            Log.broadcast( null, true, 'a' );
            Log.broadcast({}, 'b' );

            expect( consoleSpies.log ).toHaveBeenCalledTimes( 2 );
        });
    });

    /*
        Log > forwarding to renderer windows (main process)
    */

    describe( 'Forwarding to renderer windows', () =>
    {
        it( 'should send logs to every live window from the main process', async() =>
        {
            const live = { webContents: { isDestroyed: () => false, send: vi.fn() } };
            const destroyed = { webContents: { isDestroyed: () => true, send: vi.fn() } };
            mocks.getAllWindows = vi.fn( () => [ live, destroyed, null, {}] );

            const { default: Log } = await loadLog({ processType: 'browser' });
            Log.warn( 'to renderer' );

            await vi.waitFor( () => expect( live.webContents.send ).toHaveBeenCalled() );
            expect( live.webContents.send ).toHaveBeenCalledWith( 'main-log-to-renderer', expect.objectContaining(
            {
                level: 'warn',
                message: 'to renderer',
                isSimplified: true
            }) );
            expect( destroyed.webContents.send ).not.toHaveBeenCalled();
        });

        it( 'should log an error if windows cannot be listed', async() =>
        {
            mocks.getAllWindows = vi.fn()
                .mockImplementationOnce( () =>
                {
                    throw new Error( 'no windows' );
                })
                .mockImplementation( () => [] );

            const { default: Log } = await loadLog({ processType: 'browser' });
            Log.warn( 'x' );

            await vi.waitFor( () => expect( mocks.log.error ).toHaveBeenCalledWith( '[error]', 'Failed to send log to renderer: no windows' ) );
        });

        it( 'should not forward outside the main process', async() =>
        {
            const { default: Log } = await loadLog({ processType: 'renderer' });
            Log.warn( 'x' );
            await flush();

            expect( mocks.getAllWindows ).not.toHaveBeenCalled();
        });
    });

    /*
        Log > initializeLogSystem
    */

    describe( 'initializeLogSystem()', () =>
    {
        it( 'should not auto-initialize under NODE_ENV=test', async() =>
        {
            await loadLog();

            expect( mocks.log.initialize ).not.toHaveBeenCalled();
        });

        it( 'should auto-initialize outside of tests', async() =>
        {
            await loadLog({ env: { NODE_ENV: 'production' } });

            await vi.waitFor( () => expect( mocks.log.initialize ).toHaveBeenCalled() );
            await flush();
            expect( mocks.log.transports.remote.level ).toBe( false );
        });

        it( 'should configure the file transport in the main process', async() =>
        {
            mocks.existsSync.mockReturnValue( false );

            /*
                LOG_LEVEL=4 keeps Log.debug quiet: with debug on, every message also forwards to
                renderer windows via a concurrent import( 'electron' ), and vitest can hand one of
                those concurrent imports the real package instead of the mock
            */

            const { initializeLogSystem } = await loadLog({ processType: 'browser', env: { LOG_LEVEL: '4' } });
            const debug = mocks.log.debug;

            await expect( initializeLogSystem() ).resolves.toBe( true );
            await waitForInitTimer();

            const file = mocks.log.transports.file;
            expect( mocks.existsSync ).toHaveBeenCalled();
            expect( mocks.mkdirSync ).toHaveBeenCalledWith( expect.stringContaining( 'logs' ), { recursive: true });
            expect( file.resolvePathFn() ).toMatch( /logs[\\/]main\.log$/ );
            expect( file.fileName ).toBe( 'main.log' );

            // NODE_ENV=test: every transport off and electron-log methods replaced with no-ops
            expect( file.level ).toBe( false );
            expect( mocks.log.transports.console.level ).toBe( false );
            expect( mocks.log.transports.ipc.level ).toBe( false );
            expect( mocks.log.transports.remote.level ).toBe( false );
            expect( mocks.log.debug ).not.toBe( debug );
            for ( const method of [ 'error', 'warn', 'info', 'debug', 'verbose', 'silly' ] )
                expect( mocks.log[ method ]() ).toBeUndefined();
        });

        it( 'should reuse an existing log directory', async() =>
        {
            const { initializeLogSystem } = await loadLog({ processType: 'browser' });

            await expect( initializeLogSystem() ).resolves.toBe( true );
            expect( mocks.mkdirSync ).not.toHaveBeenCalled();
        });

        it( 'should warn when the main process has no file transport', async() =>
        {
            delete mocks.log.transports.file;
            delete mocks.log.transports.console;
            delete mocks.log.transports.ipc;
            delete mocks.log.transports.remote;
            delete mocks.log.initialize;

            const { initializeLogSystem } = await loadLog({ processType: 'browser' });
            const warn = mocks.log.warn;

            await expect( initializeLogSystem() ).resolves.toBe( true );
            expect( warn ).toHaveBeenCalledWith( '[warn]', 'File transport not available in main process' );
        });

        it( 'should fall back to the default file path if the app path is unavailable', async() =>
        {
            mocks.app.getAppPath = vi.fn( () =>
            {
                throw new Error( 'no app' );
            });

            const { initializeLogSystem } = await loadLog({ processType: 'browser' });
            const warn = mocks.log.warn;

            await expect( initializeLogSystem() ).resolves.toBe( true );
            expect( warn ).toHaveBeenCalledWith( '[warn]', 'Could not configure main process logging: no app' );
        });

        it( 'should handle a missing file transport in the fallback path', async() =>
        {
            mocks.app.getAppPath = vi.fn( () =>
            {
                throw new Error( 'no app' );
            });
            mocks.existsSync.mockImplementation( () =>
            {
                delete mocks.log.transports.file;
                return true;
            });

            const { initializeLogSystem } = await loadLog({ processType: 'browser' });

            await expect( initializeLogSystem() ).resolves.toBe( true );
        });

        it( 'should disable the console and IPC transports in the renderer', async() =>
        {
            const { initializeLogSystem } = await loadLog({ processType: 'renderer', env: { LOG_LEVEL: '5' } });
            const debug = mocks.log.debug;

            await expect( initializeLogSystem() ).resolves.toBe( true );
            await waitForInitTimer();

            expect( mocks.log.transports.console.level ).toBe( false );
            expect( mocks.log.transports.ipc.level ).toBe( false );
            expect( debug ).toHaveBeenCalledWith( '[debug]', 'Disabled electron-log IPC transport - using custom Log class IPC system' );
        });

        it( 'should cope with a renderer that has no console or IPC transport', async() =>
        {
            mocks.log.transports = { file: { level: 'info' } };

            const { initializeLogSystem } = await loadLog({ processType: 'renderer' });

            await expect( initializeLogSystem() ).resolves.toBe( true );
        });

        it( 'should only disable the remote transport outside of tests', async() =>
        {
            const { initializeLogSystem } = await loadLog({ processType: 'renderer' });
            process.env.NODE_ENV = 'production';

            await expect( initializeLogSystem() ).resolves.toBe( true );
            expect( mocks.log.transports.remote.level ).toBe( false );
            expect( mocks.log.transports.file.level ).toBe( 'info' );

            delete mocks.log.transports.remote;
            await expect( initializeLogSystem() ).resolves.toBe( true );
        });

        it( 'should default LOG_LEVEL to 4', async() =>
        {
            const { initializeLogSystem } = await loadLog({ env: { LOG_LEVEL: undefined } });

            await initializeLogSystem();

            expect( process.env.LOG_LEVEL ).toBe( '4' );
        });

        it( 'should return false when electron-log has no transports', async() =>
        {
            delete mocks.log.transports;

            const { initializeLogSystem } = await loadLog();

            await expect( initializeLogSystem() ).resolves.toBe( false );
            expect( mocks.log.warn ).toHaveBeenCalledWith( '[warn]', 'electron-log transports not available, using fallback' );
        });

        it( 'should warn about the console fallback in development', async() =>
        {
            delete mocks.log.transports;

            const { initializeLogSystem } = await loadLog();
            process.env.NODE_ENV = 'development';

            await expect( initializeLogSystem() ).resolves.toBe( false );
            expect( mocks.log.warn ).toHaveBeenCalledWith( '[warn]', 'Using console fallback for logging' );
        });

        it.each( [
            [ 'test', { NODE_ENV: 'test' }, false ],
            [ 'development', { NODE_ENV: 'development' }, true ],
            [ 'DEV_MODE', { NODE_ENV: 'staging', DEV_MODE: 'true' }, true ],
            [ 'production', { NODE_ENV: 'production', DEV_MODE: undefined }, false ]
        ] )( 'should return false if electron-log throws (%s)', async( label, env, warns ) =>
        {
            mocks.log.initialize = vi.fn( () =>
            {
                throw new Error( 'init failed' );
            });

            const { initializeLogSystem } = await loadLog();
            await flush();
            mocks.log.warn = vi.fn();
            for ( const [ key, value ] of Object.entries( env ) )
            {
                if ( value === undefined )
                    delete process.env[ key ];
                else
                    process.env[ key ] = value;
            }

            await expect( initializeLogSystem() ).resolves.toBe( false );

            if ( warns )
                expect( mocks.log.warn ).toHaveBeenCalledWith( '[warn]', 'Failed to initialize electron-log: init failed' );
            else
                expect( mocks.log.warn ).not.toHaveBeenCalled();
        });
    });
});
