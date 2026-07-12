const vscode = require('vscode');

// Documentation database for built-ins, namespaces, and keywords
const docs = {
    // Keywords
    "let": "### `let`\nDeclares a mutable variable.\n\n```aly\nlet x = 10\nx = 20 # valid\n```",
    "const": "### `const`\nDeclares an immutable constant.\n\n```aly\nconst PI = 3.14\nPI = 3.15 # Error\n```",
    "fun": "### `fun`\nDeclares a function.\n\n```aly\nfun add(a, b) {\n    return a + b\n}\n```",
    "return": "### `return`\nExits a function and returns a value. Defaults to `None` if no value is specified.",
    "async": "### `async`\nDeclares an asynchronous function that returns a promise.",
    "await": "### `await`\nWaits for an asynchronous promise to resolve.",
    "loop": "### `loop`\nDeclares a loop. Can be a C-style loop or an infinite loop.\n\n```aly\nloop i = 0 ; i lt 10 ; i++ {\n    print(i)\n}\n```",
    "match": "### `match`\nPattern matching block.\n\n```aly\nmatch x {\n    1 => print(\"one\")\n    _ => print(\"other\")\n}\n```",
    "tomb": "### `tomb`\nDeclares an immutable variable binding (cannot be reassigned or mutated).",
    "import": "### `import`\nImports symbols or modules.",
    "export": "### `export`\nExports symbols or modules.",
    "new": "### `new`\nInstantiates an object or class instance.",
    "struct": "### `struct`\nDeclares a structured data type.",
    "model": "### `model`\nDeclares a class definition.",
    "schema": "### `schema`\nDeclares a data validation schema.",
    "try": "### `try`\nStarts a try-catch block for handling runtime errors.",
    "catch": "### `catch`\nCatches exceptions thrown in a try block.\n\n```aly\ntry {\n    # code\n} catch err {\n    print(err)\n}\n```",
    
    // Built-in functions
    "print": "### `print(value)`\nPrints a value to stdout followed by a newline.",
    "input": "### `input(prompt)`\nPrints a prompt and reads a line of input from standard input.",
    "pow": "### `pow(base, exp)`\nReturns the base raised to the power of the exponent.",
    "sqrt": "### `sqrt(value)`\nReturns the square root of the value.",
    "random": "### `random()`\nReturns a pseudo-random floating point number in `[0, 1)`. ",
    "round": "### `round(value)`\nRounds the number to the nearest integer.",
    "roundUp": "### `roundUp(value)`\nCeiling function. Rounds the number up to the next integer.",
    "roundDown": "### `roundDown(value)`\nRounds the number down to the next integer.",
    "to_fixed": "### `to_fixed(value, precision)`\nFormats a floating point number with `precision` decimal places and returns a string.",

    // fs
    "fs": "### `fs` (Filesystem)\nNamespace providing filesystem functions.",
    "fs.read": "### `fs.read(path)`\nReads the entire text content of a file.\n\n- **Returns**: string\n- **Example**: `let content = fs.read(\"file.txt\")`",
    "fs.write": "### `fs.write(path, text)`\nWrites text to a file (overwriting existing content).\n\n- **Returns**: boolean (success/failure)\n- **Example**: `fs.write(\"file.txt\", \"hello\")`",
    "fs.append": "### `fs.append(path, text)`\nAppends text to a file, creating it if it does not exist.\n\n- **Returns**: boolean\n- **Example**: `fs.append(\"log.txt\", \"\\nnew event\")`",
    "fs.exists": "### `fs.exists(path)`\nChecks if the specified path exists.\n\n- **Returns**: boolean",
    "fs.remove": "### `fs.remove(path)`\nDeletes a file or directory recursively.\n\n- **Returns**: boolean",
    "fs.mkdir": "### `fs.mkdir(path)`\nCreates a directory and any intermediate parent directories.\n\n- **Returns**: boolean",
    "fs.list": "### `fs.list(path)`\nLists directory entries.\n\n- **Returns**: vector (array of strings)",
    "fs.is_dir": "### `fs.is_dir(path)`\nChecks if the path is a directory.\n\n- **Returns**: boolean",

    // str
    "str": "### `str` (String manipulation)\nNamespace containing string helper functions.",
    "str.upper": "### `str.upper(s)`\nReturns the uppercase version of string `s`.",
    "str.lower": "### `str.lower(s)`\nReturns the lowercase version of string `s`.",
    "str.trim": "### `str.trim(s)`\nRemoves leading and trailing whitespace from string `s`.",
    "str.contains": "### `str.contains(s, needle)`\nChecks if `s` contains the substring `needle`.\n\n- **Returns**: boolean",
    "str.replace": "### `str.replace(s, from, to)`\nReplaces all occurrences of `from` with `to` in `s`.\n\n- **Returns**: string",
    "str.split": "### `str.split(s, sep)`\nSplits `s` into a vector of strings using `sep` as separator. Empty `sep` splits characters.\n\n- **Returns**: vector",
    "str.starts_with": "### `str.starts_with(s, prefix)`\nChecks if `s` starts with `prefix`.\n\n- **Returns**: boolean",
    "str.ends_with": "### `str.ends_with(s, suffix)`\nChecks if `s` ends with `suffix`.\n\n- **Returns**: boolean",
    "str.index_of": "### `str.index_of(s, needle)`\nReturns the byte index of `needle` in `s`, or `-1` if not found.",
    "str.repeat": "### `str.repeat(s, n)`\nRepeats string `s` `n` times.",

    // sys
    "sys": "### `sys` (System interface)\nNamespace providing platform and process environment info.",
    "sys.env": "### `sys.env(name)`\nRetrieves environment variable value (empty string if unset).\n\n- **Returns**: string",
    "sys.args": "### `sys.args()`\nReturns the command line arguments vector.",
    "sys.time": "### `sys.time()`\nReturns current Unix timestamp in seconds.\n\n- **Returns**: integer",
    "sys.platform": "### `sys.platform()`\nReturns OS platform name (`linux`, `windows`, `macos`, etc.).",
    "sys.cwd": "### `sys.cwd()`\nReturns current working directory.",
    "sys.exit": "### `sys.exit(code)`\nExits the process with the given status code.",

    // http
    "http": "### `http` (HTTP Client)\nNamespace providing HTTP client requests.",
    "http.get": "### `http.get(url)`\nSends a GET request and returns the response body.",
    "http.post": "### `http.post(url, body)`\nSends a POST request with JSON body.",
    "http.put": "### `http.put(url, body)`\nSends a PUT request with JSON body.",
    "http.delete": "### `http.delete(url)`\nSends a DELETE request.",
    "http.patch": "### `http.patch(url, body)`\nSends a PATCH request.",
    "http.request": "### `http.request(method, url, body)`\nSends custom HTTP request.",
    "http.status_code": "### `http.status_code(url)`\nSends a GET request and returns the HTTP status code (int).",
    "http.head": "### `http.head(url)`\nSends a HEAD request and returns JSON headers info."
};

// Dynamic helper to extract defined functions, variables, and models from current file
function findUserDefinitions(document) {
    const text = document.getText();
    const defs = [];
    
    // 1. Functions: fun name(a, b) or async fun name(a, b)
    const funRegex = /(?:async\s+)?fun\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(([^)]*)\)/g;
    let match;
    while ((match = funRegex.exec(text)) !== null) {
        const name = match[1];
        const params = match[2];
        const pos = document.positionAt(match.index);
        defs.push({
            name,
            kind: 'function',
            signature: `fun ${name}(${params.trim()})`,
            description: `User-defined function in this file.`,
            position: pos
        });
    }
    
    // 2. Variables & Constants: let x = ... or const y = ...
    const varRegex = /\b(let|const)\s+([a-zA-Z_][a-zA-Z0-9_]*)\b/g;
    while ((match = varRegex.exec(text)) !== null) {
        const type = match[1];
        const name = match[2];
        const pos = document.positionAt(match.index);
        defs.push({
            name,
            kind: 'variable',
            signature: `${type} ${name}`,
            description: `User-defined ${type === 'const' ? 'constant' : 'variable'} in this file.`,
            position: pos
        });
    }
    
    // 3. Structs & Classes: struct Point or model Person or schema User
    const structRegex = /\b(struct|model|schema)\s+([a-zA-Z_][a-zA-Z0-9_]*)\b/g;
    while ((match = structRegex.exec(text)) !== null) {
        const type = match[1];
        const name = match[2];
        const pos = document.positionAt(match.index);
        defs.push({
            name,
            kind: 'class',
            signature: `${type} ${name}`,
            description: `User-defined ${type} in this file.`,
            position: pos
        });
    }
    
    return defs;
}

function activate(context) {
    console.log('Aly language server extension active.');

    // 1. Hover Provider
    const hoverProvider = vscode.languages.registerHoverProvider('aly', {
        provideHover(document, position, token) {
            const range = document.getWordRangeAtPosition(position);
            if (!range) return null;
            
            const line = document.lineAt(position.line).text;
            const word = document.getText(range);
            
            // Expand dotted terms (e.g. "fs.read")
            let fullExpression = word;
            const wordRegEx = /[a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z_][a-zA-Z0-9_]*)*/g;
            let match;
            while ((match = wordRegEx.exec(line)) !== null) {
                const start = match.index;
                const end = match.index + match[0].length;
                if (position.character >= start && position.character <= end) {
                    fullExpression = match[0];
                    break;
                }
            }
            
            // Check built-in docs
            if (docs[fullExpression]) {
                return new vscode.Hover(new vscode.MarkdownString(docs[fullExpression]));
            }
            
            // Check if it's a sub-part of a built-in namespace, e.g. hovering "read" in "fs.read"
            if (docs[word]) {
                return new vscode.Hover(new vscode.MarkdownString(docs[word]));
            }

            // Check user definitions in current file
            const userDefs = findUserDefinitions(document);
            const userDef = userDefs.find(d => d.name === word);
            if (userDef) {
                return new vscode.Hover([
                    `\`\`\`aly\n${userDef.signature}\n\`\`\``,
                    userDef.description
                ]);
            }
            
            return null;
        }
    });

    // 2. Autocomplete Provider (General & Namespace context)
    const completionProvider = vscode.languages.registerCompletionItemProvider('aly', {
        provideCompletionItems(document, position, token, context) {
            const completionItems = [];
            const linePrefix = document.lineAt(position).text.substr(0, position.character);
            
            // 2a. Namespace completion (e.g. typing "fs.")
            const namespaces = ['fs', 'str', 'sys', 'http'];
            for (const ns of namespaces) {
                if (linePrefix.endsWith(`${ns}.`)) {
                    Object.keys(docs).forEach(key => {
                        if (key.startsWith(`${ns}.`)) {
                            const methodName = key.substring(ns.length + 1);
                            const item = new vscode.CompletionItem(methodName, vscode.CompletionItemKind.Method);
                            item.detail = key;
                            item.documentation = new vscode.MarkdownString(docs[key]);
                            completionItems.push(item);
                        }
                    });
                    return completionItems;
                }
            }
            
            // 2b. Global built-in namespaces
            namespaces.forEach(ns => {
                const item = new vscode.CompletionItem(ns, vscode.CompletionItemKind.Module);
                item.detail = `namespace ${ns}`;
                item.documentation = new vscode.MarkdownString(docs[ns]);
                completionItems.push(item);
            });
            
            // 2c. Global built-in functions
            const globalFunctions = ['print', 'input', 'pow', 'sqrt', 'random', 'round', 'roundUp', 'roundDown', 'to_fixed'];
            globalFunctions.forEach(fn => {
                const item = new vscode.CompletionItem(fn, vscode.CompletionItemKind.Function);
                item.detail = `built-in function ${fn}`;
                item.documentation = new vscode.MarkdownString(docs[fn]);
                completionItems.push(item);
            });
            
            // 2d. Keywords
            const keywords = [
                'let', 'const', 'fun', 'return', 'if', 'elif', 'else',
                'loop', 'do', 'match', 'tomb', 'import', 'export', 'new',
                'struct', 'model', 'try', 'catch', 'async', 'await', 'schema'
            ];
            keywords.forEach(kw => {
                const item = new vscode.CompletionItem(kw, vscode.CompletionItemKind.Keyword);
                item.documentation = new vscode.MarkdownString(docs[kw] || `Keyword: ${kw}`);
                completionItems.push(item);
            });

            // 2e. User definitions
            const userDefs = findUserDefinitions(document);
            userDefs.forEach(def => {
                const item = new vscode.CompletionItem(def.name);
                if (def.kind === 'function') {
                    item.kind = vscode.CompletionItemKind.Function;
                    item.detail = `(User) ${def.signature}`;
                } else if (def.kind === 'class') {
                    item.kind = vscode.CompletionItemKind.Class;
                    item.detail = `(User) ${def.signature}`;
                } else {
                    item.kind = vscode.CompletionItemKind.Variable;
                    item.detail = `(User) ${def.signature}`;
                }
                item.documentation = def.description;
                completionItems.push(item);
            });
            
            return completionItems;
        }
    }, '.'); // Trigger autocomplete on '.'

    // 3. Go-to-Definition Provider (F12 / Ctrl+Click)
    const definitionProvider = vscode.languages.registerDefinitionProvider('aly', {
        provideDefinition(document, position, token) {
            const range = document.getWordRangeAtPosition(position);
            if (!range) return null;
            
            const word = document.getText(range);
            const userDefs = findUserDefinitions(document);
            const userDef = userDefs.find(d => d.name === word);
            
            if (userDef) {
                return new vscode.Location(document.uri, userDef.position);
            }
            
            return null;
        }
    });

    context.subscriptions.push(hoverProvider, completionProvider, definitionProvider);
}

function deactivate() {}

module.exports = {
    activate,
    deactivate
};
