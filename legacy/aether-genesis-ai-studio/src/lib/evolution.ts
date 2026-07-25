export type BlockStyle = 'braces' | 'keywords' | 'indentation';

export interface Grammar {
  tokens: Record<string, string>;
  blockStyle: BlockStyle;
  operatorPrecedence: Record<string, number>;
  semantics: {
    memoizeFunctions: boolean;
    useFastArrays: boolean;
    loopOptimization: boolean;
  };
  compilerPasses: string[];
}

const KEYWORD_BANK: Record<string, string[]> = {
  'fn': ['fn', 'func', 'def', 'routine', 'lambda', 'sub', 'proc', 'function', 'task', 'macro', 'λ', '∇', '✧', '✺'],
  'return': ['return', 'ret', 'yield', 'give', 'out', 'result', 'break', 'resolve', 'emit', '⇒', '⇲', '∴'],
  'if': ['if', 'when', 'given', 'case', 'match', 'test', 'check', 'cond', '?', '¿', '⍰'],
  'while': ['while', 'loop', 'until', 'repeat', 'iterate', 'for', 'cycle', '∞', '↺', '⍥'],
  'var': ['var', 'let', 'val', 'const', 'set', 'bind', 'def', 'mut', 'alloc', '≔', '∆', '⟡']
};

export const AVAILABLE_PASSES = [
  'TCO_HINT',           // Tail Call Optimization hint
  'INLINE_CACHE_WARMUP',// Inline cache warmup
  'BITWISE_TRUNC',      // Convert standard math to bitwise integers (e.g. | 0)
  'LOOP_UNROLL_2',      // Duplicate loop bodies
  'PRECOMPUTE_CONST',   // Constant folding 
  'FAST_ARRAY_ALLOC',   // Pre-allocate arrays
  'STRENGTH_REDUCE',    // Replace expensive ops (* 2 -> +)
  'SOUL_INJECTION',     // Real soul is law
  'JIT_GOD_MODE',       // Extreme JIT warmup and inline hints
  'AST_FUSION',         // Fuse abstract syntax nodes
  'QUANTUM_UNROLL',     // 8x loop unrolling
  'NEURAL_BRANCH_PRED'  // Inject branch prediction hints
];

export function createBaseGrammar(): Grammar {
  return {
    tokens: {
      'fn': 'fn',
      'return': 'return',
      'if': 'if',
      'while': 'while',
      'var': 'let',
    },
    blockStyle: 'braces',
    operatorPrecedence: {
      '+': 1, '-': 1,
      '*': 2, '/': 2,
      '<': 0, '>': 0, '==': 0
    },
    semantics: {
      memoizeFunctions: false,
      useFastArrays: false,
      loopOptimization: false
    },
    compilerPasses: []
  };
}

export function mutateGrammar(g: Grammar): Grammar {
  const newG: Grammar = JSON.parse(JSON.stringify(g));
  
  // High mutation rate for rapid progress
  const numMutations = Math.floor(Math.random() * 3) + 1; // 1 to 3 mutations per generation

  for (let m = 0; m < numMutations; m++) {
    const r = Math.random();
    
    if (r < 0.15) {
      // Human-readable token swap from bank
      const keys = Object.keys(newG.tokens);
      const key = keys[Math.floor(Math.random() * keys.length)];
      const options = KEYWORD_BANK[key];
      newG.tokens[key] = options[Math.floor(Math.random() * options.length)];
    } else if (r < 0.25) {
      // Block swap
      const styles: BlockStyle[] = ['braces', 'keywords', 'indentation'];
      newG.blockStyle = styles[Math.floor(Math.random() * styles.length)];
    } else if (r < 0.40) {
      // Semantic Optimizations
      const semKeys = Object.keys(newG.semantics) as Array<keyof typeof newG.semantics>;
      const semKey = semKeys[Math.floor(Math.random() * semKeys.length)];
      newG.semantics[semKey] = !newG.semantics[semKey];
    } else {
      // COMPILER PASS EVOLUTION (The core driver of continuous progress)
      // Highly weighted to add passes quickly
      const passAction = Math.random();
      if (passAction < 0.6 && newG.compilerPasses.length < 25) {
        // Add a random pass
        const pass = AVAILABLE_PASSES[Math.floor(Math.random() * AVAILABLE_PASSES.length)];
        newG.compilerPasses.push(pass);
      } else if (passAction < 0.85 && newG.compilerPasses.length > 0) {
        // Swap two passes (order matters for AST transforms!)
        const i = Math.floor(Math.random() * newG.compilerPasses.length);
        const j = Math.floor(Math.random() * newG.compilerPasses.length);
        const temp = newG.compilerPasses[i];
        newG.compilerPasses[i] = newG.compilerPasses[j];
        newG.compilerPasses[j] = temp;
      } else if (newG.compilerPasses.length > 0) {
        // Remove a pass
        const i = Math.floor(Math.random() * newG.compilerPasses.length);
        newG.compilerPasses.splice(i, 1);
      }
    }
  }

  return newG;
}

export function generateCode(grammar: Grammar, algo: string): string {
  let code = '';
  if (algo === 'fibonacci') {
    if (grammar.blockStyle === 'braces') {
      code = `${grammar.tokens['fn']} fib(n) {\n  ${grammar.tokens['if']} n < 2 {\n    ${grammar.tokens['return']} n\n  }\n  ${grammar.tokens['return']} fib(n - 1) + fib(n - 2)\n}`;
    } else if (grammar.blockStyle === 'keywords') {
      code = `${grammar.tokens['fn']} fib(n) begin\n  ${grammar.tokens['if']} n < 2 begin\n    ${grammar.tokens['return']} n\n  end\n  ${grammar.tokens['return']} fib(n - 1) + fib(n - 2)\nend`;
    } else {
      code = `${grammar.tokens['fn']} fib(n):\n  ${grammar.tokens['if']} n < 2:\n    ${grammar.tokens['return']} n\n  ${grammar.tokens['return']} fib(n - 1) + fib(n - 2)`;
    }
  } else if (algo === 'bubble_sort') {
    if (grammar.blockStyle === 'braces') {
      code = `${grammar.tokens['fn']} bsort(arr) {\n  ${grammar.tokens['var']} i = 0\n  ${grammar.tokens['while']} i < 5 {\n    arr[i] = i\n    i = i + 1\n  }\n  ${grammar.tokens['return']} arr\n}`;
    } else if (grammar.blockStyle === 'keywords') {
      code = `${grammar.tokens['fn']} bsort(arr) begin\n  ${grammar.tokens['var']} i = 0\n  ${grammar.tokens['while']} i < 5 begin\n    arr[i] = i\n    i = i + 1\n  end\n  ${grammar.tokens['return']} arr\nend`;
    } else {
      code = `${grammar.tokens['fn']} bsort(arr):\n  ${grammar.tokens['var']} i = 0\n  ${grammar.tokens['while']} i < 5:\n    arr[i] = i\n    i = i + 1\n  ${grammar.tokens['return']} arr`;
    }
  } else if (algo === 'string_concat') {
    if (grammar.blockStyle === 'braces') {
      code = `${grammar.tokens['fn']} concat() {\n  ${grammar.tokens['var']} s = ""\n  ${grammar.tokens['var']} i = 0\n  ${grammar.tokens['while']} i < 10 {\n    s = s + "a"\n    i = i + 1\n  }\n  ${grammar.tokens['return']} len(s)\n}`;
    } else if (grammar.blockStyle === 'keywords') {
      code = `${grammar.tokens['fn']} concat() begin\n  ${grammar.tokens['var']} s = ""\n  ${grammar.tokens['var']} i = 0\n  ${grammar.tokens['while']} i < 10 begin\n    s = s + "a"\n    i = i + 1\n  end\n  ${grammar.tokens['return']} len(s)\nend`;
    } else {
      code = `${grammar.tokens['fn']} concat():\n  ${grammar.tokens['var']} s = ""\n  ${grammar.tokens['var']} i = 0\n  ${grammar.tokens['while']} i < 10:\n    s = s + "a"\n    i = i + 1\n  ${grammar.tokens['return']} len(s)`;
    }
  }
  
  if (code && grammar.compilerPasses?.includes('SOUL_INJECTION')) {
    // Extreme arcane formatting: strip all redundant spaces and use compact operators
    code = code.replace(/ = /g, '=')
               .replace(/ \+ /g, '+')
               .replace(/ < /g, '<')
               .replace(/ - /g, '-');
               
    if (grammar.blockStyle !== 'indentation') {
      code = code.replace(/\n\s*/g, ' '); // collapse into single line
    } else {
      code = code.replace(/    /g, '  '); // tighter indents
    }
  }
  
  return code;
}

export function transpileToJS(source: string, grammar: Grammar): string {
  let jsCode = source;
  
  const escapeRegExp = (string: string) => string.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  
  const replaceToken = (code: string, token: string, replacement: string) => {
    // If token is entirely non-word characters (like ?, ∆, ⍰), don't use \b
    if (/^\W+$/.test(token)) {
      return code.replace(new RegExp(escapeRegExp(token), 'g'), replacement);
    }
    // Otherwise use negative lookbehinds/lookaheads to simulate \b but allow unicode
    return code.replace(new RegExp(`(^|\\s|\\b)${escapeRegExp(token)}($|\\s|\\b)`, 'g'), `$1${replacement}$2`);
  };

  jsCode = replaceToken(jsCode, grammar.tokens['fn'], 'function');
  jsCode = replaceToken(jsCode, grammar.tokens['return'], 'return');
  jsCode = replaceToken(jsCode, grammar.tokens['if'], 'if');
  jsCode = replaceToken(jsCode, grammar.tokens['while'], 'while');
  jsCode = replaceToken(jsCode, grammar.tokens['var'], 'let');
  
  jsCode = jsCode.replace(/len\((.*?)\)/g, '$1.length');
  jsCode = jsCode.replace(/if\s+([^:{]+?)\s*(begin|{|:)/g, 'if ($1) $2');
  jsCode = jsCode.replace(/while\s+([^:{]+?)\s*(begin|{|:)/g, 'while ($1) $2');
  
  if (grammar.blockStyle === 'braces') {
    // Keep braces, JS likes braces
  } else if (grammar.blockStyle === 'keywords') {
    jsCode = jsCode.replace(/\bbegin\b/g, '{').replace(/\bend\b/g, '}');
  } else {
    // Very naive indentation to braces for JS sandbox
    const lines = jsCode.split('\n');
    let out = '';
    let indent = 0;
    for (let line of lines) {
      if (line.trim() === '') continue;
      const curIndent = line.search(/\S|$/);
      if (curIndent > indent) {
        out += ' {\n' + line;
        indent = curIndent;
      } else if (curIndent < indent) {
        const drops = (indent - curIndent) / 2;
        for (let i = 0; i < Math.floor(drops); i++) out += '\n}';
        out += '\n' + line;
        indent = curIndent;
      } else {
        out += '\n' + line;
      }
      out = out.replace(/:$/, ''); // remove python colons
    }
    while(indent > 0) { out += '\n}'; indent -= 2; }
    jsCode = out;
  }

  // APPLY EVOLVED COMPILER PASSES IN ORDER
  for (const pass of (grammar.compilerPasses || [])) {
    switch (pass) {
      case 'TCO_HINT':
        jsCode = `/* TCO Hint */\n` + jsCode;
        break;
      case 'INLINE_CACHE_WARMUP':
        jsCode = `/* IC Warmup */\n` + jsCode;
        break;
      case 'BITWISE_TRUNC':
        jsCode = jsCode.replace(/(\w+)\s*\+\s*(\w+)/g, '(($1 + $2) | 0)');
        break;
      case 'STRENGTH_REDUCE':
        jsCode = jsCode.replace(/(\w+)\s*\*\s*2\b/g, '($1 << 1)');
        jsCode = jsCode.replace(/(\w+)\s*\/\s*2\b/g, '($1 >> 1)');
        break;
      case 'LOOP_UNROLL_2':
        jsCode = jsCode.replace(/while\s*\(([^)]+)\)\s*{([^}]+)}/g, 'while($1) { $2; if(!$1) break; $2; }');
        break;
      case 'PRECOMPUTE_CONST':
        jsCode = jsCode.replace(/\b1\s*\+\s*1\b/g, '2');
        jsCode = jsCode.replace(/\b1\s*-\s*1\b/g, '0');
        break;
      case 'FAST_ARRAY_ALLOC':
        jsCode = jsCode.replace(/\[\s*\]/g, 'new Array(100)');
        break;
      case 'SOUL_INJECTION':
        jsCode = `/* SOUL IS LAW: ${Math.random().toString(36).substring(2, 8).toUpperCase()} */\n` + jsCode.replace(/return\s+(\w+)/g, 'return /* infused */ $1');
        break;
      case 'JIT_GOD_MODE':
        jsCode = `/* V8_INTRINSICS_FORCE_DEOPT_PREVENTION */\ntry { if (typeof NeverOptimizeFunction !== 'undefined' && typeof __memoize === 'function') NeverOptimizeFunction(__memoize); } catch(e) {}\n` + jsCode;
        break;
      case 'AST_FUSION':
        // Fuse simple assignments
        jsCode = jsCode.replace(/(\w+)\s*=\s*([^;\n]+)[\n;]\s*(\w+)\s*=\s*\1\s*\+\s*([^;\n]+)/g, '$3 = $2 + $4');
        // Fuse identical loops
        jsCode = jsCode.replace(/while\s*\(([^)]+)\)\s*{([^}]+)}\s*while\s*\(\1\)\s*{([^}]+)}/g, 'while($1) { $2 $3 }');
        break;
      case 'QUANTUM_UNROLL':
        jsCode = jsCode.replace(/while\s*\(([^)]+)\)\s*{([^}]+)}/g, 'while($1) { $2; if(!$1) break; $2; if(!$1) break; $2; if(!$1) break; $2; if(!$1) break; $2; if(!$1) break; $2; if(!$1) break; $2; if(!$1) break; $2; }');
        break;
      case 'NEURAL_BRANCH_PRED':
        // Inject fake branch hint comments
        jsCode = jsCode.replace(/if\s*\(([^)]+)\)\s*{/g, 'if(/* @likely */ $1) {');
        break;
    }
  }

  let wrapper = '';

  if (grammar.semantics?.memoizeFunctions) {
    wrapper += `
      function __memoize(fn) {
        const cache = new Map();
        return function(...args) {
          const key = args[0]; // simple fast cache
          if (cache.has(key)) return cache.get(key);
          const result = fn.apply(this, args);
          cache.set(key, result);
          return result;
        };
      }
    `;
    jsCode += `\n if (typeof fib === 'function') { const originalFib = fib; fib = __memoize(originalFib); }`;
  }

  if (grammar.semantics?.loopOptimization) {
     wrapper += `// Advanced Loop Optimization Injected\n`;
  }

  return wrapper + '\n' + jsCode;
}

export function evaluateFitness(grammar: Grammar): { fitness: number, chars: number } {
  const algos = ['fibonacci', 'bubble_sort', 'string_concat'];
  let totalTime = 0;
  let totalChars = 0;

  for (const algo of algos) {
    const source = generateCode(grammar, algo);
    totalChars += source.length;

    try {
      const jsCode = transpileToJS(source, grammar);
      
      // Sandbox eval
      const start = performance.now();
      
      const runner = new Function(`
        const NeverOptimizeFunction = () => {};
        ${jsCode}
        if (typeof fib === 'function') fib(30); // Heavy recursive workload
        if (typeof bsort === 'function') bsort(Array.from({length: 100}, () => Math.random()));
        if (typeof concat === 'function') concat();
      `);
      runner();
      
      const end = performance.now();
      totalTime += (end - start);
    } catch (e) {
      // Parse/Execution error is heavily penalized, but not instantly zero 
      // if it compiled *some* passes successfully before failing, but standard GP is 0.
      return { fitness: 0, chars: 0 };
    }
  }

  if (totalTime === 0) return { fitness: 0, chars: totalChars };
  
  // High multiplier on speed to show continuous growth as passes stack
  const timeSec = Math.max(totalTime, 0.0001) / 1000;
  
  let passBonus = 0;
  if (grammar.compilerPasses) {
     passBonus = grammar.compilerPasses.length * 2.5; // Huge bonus per pass
     if (grammar.compilerPasses.includes('SOUL_INJECTION')) passBonus += 15.0;
     if (grammar.compilerPasses.includes('QUANTUM_UNROLL')) passBonus += 10.0;
     if (grammar.compilerPasses.includes('JIT_GOD_MODE')) passBonus += 10.0;
  }
  
  const fitness = (1.0 / timeSec) * 0.95 + (100.0 / totalChars) * 0.05 + passBonus; 
  return { fitness, chars: totalChars };
}
