import React, { useState, useEffect, useRef } from 'react';
import { Play, Code2, Cpu, Activity, FileJson, FastForward, Terminal } from 'lucide-react';
import { Grammar, createBaseGrammar, evaluateFitness, mutateGrammar, generateCode, transpileToJS } from './lib/evolution';

const PYTHON_CODE = `import time
import random
import re
import copy
import traceback

class LanguageGrammar:
    def __init__(self):
        # Base grammar configuration using a dictionary structure
        self.tokens = {
            'fn': 'fn',
            'return': 'return',
            'if': 'if',
            'while': 'while',
            'var': 'let',
        }
        self.block_style = 'braces' # Options: 'braces', 'keywords', 'indentation'
        self.operator_precedence = {
            '+': 1, '-': 1,
            '*': 2, '/': 2,
            '<': 0, '>': 0, '==': 0
        }

    def mutate(self):
        """
        Programmatic syntax changes without human intervention.
        Implements 3 distinct mutation functions.
        """
        mutation_type = random.choice(['token_swap', 'block_swap', 'operator_drift'])
        
        if mutation_type == 'token_swap':
            # Alter syntax tokens (e.g., change 'fn' to random symbol)
            k = random.choice(list(self.tokens.keys()))
            new_token = "".join(random.choices("abcdefghijklmnopqrstuvwxyz", k=random.randint(2, 5)))
            self.tokens[k] = new_token
            
        elif mutation_type == 'block_swap':
            # Toggle between explicit block markers and indentation
            self.block_style = random.choice(['braces', 'keywords', 'indentation'])
            
        elif mutation_type == 'operator_drift':
            # Re-order internal operator priority mappings
            k = random.choice(list(self.operator_precedence.keys()))
            self.operator_precedence[k] = random.randint(0, 3)

class CodeGenerator:
    """Auto-generates implementations of benchmark algorithms using the minted syntax rules."""
    @staticmethod
    def generate(grammar, algo):
        if algo == 'fibonacci':
            if grammar.block_style == 'braces':
                return f"{grammar.tokens['fn']} fib(n) {{\\n  {grammar.tokens['if']} n < 2 {{\\n    {grammar.tokens['return']} n\\n  }}\\n  {grammar.tokens['return']} fib(n - 1) + fib(n - 2)\\n}}"
            elif grammar.block_style == 'keywords':
                return f"{grammar.tokens['fn']} fib(n) begin\\n  {grammar.tokens['if']} n < 2 begin\\n    {grammar.tokens['return']} n\\n  end\\n  {grammar.tokens['return']} fib(n - 1) + fib(n - 2)\\nend"
            else:
                return f"{grammar.tokens['fn']} fib(n):\\n  {grammar.tokens['if']} n < 2:\\n    {grammar.tokens['return']} n\\n  {grammar.tokens['return']} fib(n - 1) + fib(n - 2)"
        
        elif algo == 'bubble_sort':
            # Simplified pseudo-bubble sort loop to test array manipulation and loops
            if grammar.block_style == 'braces':
                return f"{grammar.tokens['fn']} bsort(arr) {{\\n  {grammar.tokens['var']} i = 0\\n  {grammar.tokens['while']} i < 5 {{\\n    arr[i] = i\\n    i = i + 1\\n  }}\\n  {grammar.tokens['return']} arr\\n}}"
            elif grammar.block_style == 'keywords':
                return f"{grammar.tokens['fn']} bsort(arr) begin\\n  {grammar.tokens['var']} i = 0\\n  {grammar.tokens['while']} i < 5 begin\\n    arr[i] = i\\n    i = i + 1\\n  end\\n  {grammar.tokens['return']} arr\\nend"
            else:
                return f"{grammar.tokens['fn']} bsort(arr):\\n  {grammar.tokens['var']} i = 0\\n  {grammar.tokens['while']} i < 5:\\n    arr[i] = i\\n    i = i + 1\\n  {grammar.tokens['return']} arr"

        elif algo == 'string_concat':
            # String Manipulation loop (memory/allocation behavior test)
            if grammar.block_style == 'braces':
                return f"{grammar.tokens['fn']} concat() {{\\n  {grammar.tokens['var']} s = \\"\\"\\n  {grammar.tokens['var']} i = 0\\n  {grammar.tokens['while']} i < 10 {{\\n    s = s + \\"a\\"\\n    i = i + 1\\n  }}\\n  {grammar.tokens['return']} len(s)\\n}}"
            elif grammar.block_style == 'keywords':
                return f"{grammar.tokens['fn']} concat() begin\\n  {grammar.tokens['var']} s = \\"\\"\\n  {grammar.tokens['var']} i = 0\\n  {grammar.tokens['while']} i < 10 begin\\n    s = s + \\"a\\"\\n    i = i + 1\\n  end\\n  {grammar.tokens['return']} len(s)\\nend"
            else:
                return f"{grammar.tokens['fn']} concat():\\n  {grammar.tokens['var']} s = \\"\\"\\n  {grammar.tokens['var']} i = 0\\n  {grammar.tokens['while']} i < 10:\\n    s = s + \\"a\\"\\n    i = i + 1\\n  {grammar.tokens['return']} len(s)"

class Compiler:
    def __init__(self, grammar):
        self.grammar = grammar
        
    def transpile(self, source):
        """
        Automated parser/transpiler engine that translates code written in the mutated language 
        directly into standard, executable Python code. Bypasses manual machine-code emission.
        """
        py_code = source
        
        # Translate tokens back to Python AST/execution equivalents
        py_code = re.sub(rf"\\b{self.grammar.tokens['fn']}\\b", 'def', py_code)
        py_code = re.sub(rf"\\b{self.grammar.tokens['return']}\\b", 'return', py_code)
        py_code = re.sub(rf"\\b{self.grammar.tokens['if']}\\b", 'if', py_code)
        py_code = re.sub(rf"\\b{self.grammar.tokens['while']}\\b", 'while', py_code)
        py_code = re.sub(rf"\\b{self.grammar.tokens['var']}\\b\\s+", '', py_code)
        
        # Resolve block delimiters
        if self.grammar.block_style == 'braces':
            py_code = py_code.replace('{', ':')
            py_code = py_code.replace('}', '')
        elif self.grammar.block_style == 'keywords':
            py_code = py_code.replace('begin', ':')
            py_code = py_code.replace('end', '')
            
        return "\\n".join([line for line in py_code.split('\\n') if line.strip() != ''])

class FitnessEvaluator:
    def __init__(self, grammar):
        self.grammar = grammar
        self.compiler = Compiler(grammar)
        
    def evaluate(self):
        """Runs the Automated Benchmark Sandbox and calculates fitness."""
        algos = ['fibonacci', 'bubble_sort', 'string_concat']
        total_time = 0
        total_chars = 0
        
        for algo in algos:
            source = CodeGenerator.generate(self.grammar, algo)
            total_chars += len(source)
            
            try:
                # Compile to Python
                py_code = self.compiler.transpile(source)
                
                # Secure execution wrapper
                sandbox = {}
                exec(py_code, sandbox)
                
                # Measure performance
                start = time.perf_counter()
                if algo == 'fibonacci':
                    sandbox['fib'](10)
                elif algo == 'bubble_sort':
                    sandbox['bsort']([0]*5)
                elif algo == 'string_concat':
                    sandbox['concat']()
                end = time.perf_counter()
                
                total_time += (end - start)
            except Exception:
                # Any parsing or execution error sets fitness to 0
                return 0, 0
                
        if total_time == 0:
            return 0, total_chars
            
        # Strict mathematical fitness function
        fitness = (1.0 / total_time) * 0.5 + (1.0 / total_chars) * 0.5
        return fitness, total_chars

def run_evolution():
    """The Evolution Engine loop maintaining a population of 10 languages."""
    population = [LanguageGrammar() for _ in range(10)]
    
    print("--- INITIATING GENETIC LANGUAGE EVOLUTION ---")
    
    for generation in range(1, 101):
        scores = []
        for g in population:
            evaluator = FitnessEvaluator(g)
            fitness, chars = evaluator.evaluate()
            scores.append((fitness, chars, g))
            
        # Sort by fitness score
        scores.sort(key=lambda x: x[0], reverse=True)
        top_fitness, top_chars, top_grammar = scores[0]
        
        # Tracking log
        print(f"Gen {generation} | Top Fitness: {top_fitness:.2f} | Chars: {top_chars} | Block: {top_grammar.block_style} | Tokens: {top_grammar.tokens}")
        
        # Discard bottom 5, keep top 5
        survivors = [s[2] for s in scores[:5]]
        
        # Replace bottom 5 by cloning and mutating the top 5 survivors
        next_gen = survivors.copy()
        for s in survivors:
            clone = copy.deepcopy(s)
            clone.mutate()
            next_gen.append(clone)
            
        population = next_gen
        time.sleep(0.05)

if __name__ == "__main__":
    run_evolution()
`;

export default function App() {
  const [activeTab, setActiveTab] = useState<'python' | 'live'>('live');
  
  // GP Engine State
  const [isRunning, setIsRunning] = useState(false);
  const [isUncapped, setIsUncapped] = useState(false);
  const [generation, setGeneration] = useState(0);
  const [population, setPopulation] = useState<Grammar[]>([]);
  const [bestGrammar, setBestGrammar] = useState<Grammar | null>(null);
  const [bestFitness, setBestFitness] = useState(0);
  const [logs, setLogs] = useState<string[]>([]);
  
  const logsEndRef = useRef<HTMLDivElement>(null);
  const loopRef = useRef<number | null>(null);

  // Auto-scroll logs
  useEffect(() => {
    if (logsEndRef.current && !isUncapped) {
      logsEndRef.current.scrollIntoView({ behavior: 'auto' });
    }
  }, [logs, isUncapped]);

  const initPopulation = () => {
    const pop = Array(25).fill(null).map(() => createBaseGrammar()); // Larger population for max entropy
    setPopulation(pop);
    setGeneration(0);
    setBestFitness(0);
    setBestGrammar(null);
    setLogs(['[SYSTEM] Initialized population of 25 base grammars.']);
  };

  const stepEvolution = () => {
    setPopulation(currentPop => {
      // Evaluate all
      const evaluated = currentPop.map(g => {
        const { fitness, chars } = evaluateFitness(g);
        return { grammar: g, fitness, chars };
      });

      // Sort
      evaluated.sort((a, b) => b.fitness - a.fitness);
      
      const best = evaluated[0];
      
      setBestFitness(prev => {
        if (best.fitness > prev || prev === 0) {
          setTimeout(() => setBestGrammar(best.grammar), 0);
          
          // Log immediately if we hit a new record
          setGeneration(gen => {
            setLogs(l => {
              const passStr = best.grammar.compilerPasses?.length ? ` | Passes: ${best.grammar.compilerPasses.length}` : '';
              const soulAlert = best.grammar.compilerPasses?.includes('SOUL_INJECTION') ? ' [SOUL INFUSED]' : '';
              const newLogs = [...l, `[NEW BEST] Gen ${gen} | Top Fitness: ${best.fitness.toFixed(2)}${soulAlert} | Density: ${best.chars} chars | Block: ${best.grammar.blockStyle}${passStr}`];
              return newLogs.slice(-200); 
            });
            return gen;
          });
          
          return best.fitness;
        }
        return prev;
      });
      
      setGeneration(prev => {
        const nextGen = prev + 1;
        // Throttle log updates if uncapped to prevent UI freezing
        if (nextGen % (isUncapped ? 10 : 1) === 0) {
          setLogs(l => {
            const passStr = best.grammar.compilerPasses?.length ? ` | Passes: ${best.grammar.compilerPasses.length}` : '';
            const soulAlert = best.grammar.compilerPasses?.includes('SOUL_INJECTION') ? ' [SOUL INFUSED]' : '';
            const newLogs = [...l, `Gen ${nextGen} | Top Fitness: ${best.fitness.toFixed(2)}${soulAlert} | Density: ${best.chars} chars | Block: ${best.grammar.blockStyle}${passStr}`];
            return newLogs.slice(-200); // Keep log buffer clean
          });
        }
        return nextGen;
      });

      // Discard bottom, keep top 10
      const survivors = evaluated.slice(0, 10).map(e => e.grammar);
      
      // Clone and mutate
      const nextGeneration = [...survivors];
      for (const s of survivors) {
        nextGeneration.push(mutateGrammar(s));
        nextGeneration.push(mutateGrammar(s)); // Mutate twice to fill back to 25 pop size roughly (10 original + 20 mutants = 30 pop)
      }

      return nextGeneration.slice(0, 25);
    });
  };

  useEffect(() => {
    if (isRunning) {
      loopRef.current = window.setInterval(stepEvolution, isUncapped ? 0 : 100);
    } else if (loopRef.current) {
      clearInterval(loopRef.current);
    }
    return () => {
      if (loopRef.current) clearInterval(loopRef.current);
    };
  }, [isRunning, isUncapped]);

  return (
    <div className="flex flex-col h-screen bg-[#050505] text-zinc-300 font-sans overflow-hidden">
      {/* Header */}
      <header className="flex items-center justify-between px-6 py-3 border-b border-zinc-800 bg-[#0a0a0a] shrink-0">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded bg-blue-500/20 flex items-center justify-center border border-blue-500/30">
            <Cpu className="text-blue-400" size={18} />
          </div>
          <div>
            <h1 className="text-sm font-bold text-zinc-100 tracking-wide">AETHER<span className="text-blue-400">GENESIS</span></h1>
            <p className="text-[10px] text-zinc-500 font-mono">v1.0.0 • GENETIC EVOLUTION ENGINE</p>
          </div>
        </div>
        
        <div className="flex bg-zinc-900 rounded-lg p-1 border border-zinc-800">
          <button 
            onClick={() => setActiveTab('python')}
            className={`px-4 py-1.5 text-xs font-semibold rounded-md transition-all ${activeTab === 'python' ? 'bg-zinc-800 text-white shadow-sm' : 'text-zinc-500 hover:text-zinc-300'}`}
          >
            Python Artifact
          </button>
          <button 
            onClick={() => setActiveTab('live')}
            className={`px-4 py-1.5 text-xs font-semibold rounded-md transition-all ${activeTab === 'live' ? 'bg-zinc-800 text-white shadow-sm' : 'text-zinc-500 hover:text-zinc-300'}`}
          >
            Live Visualizer
          </button>
        </div>
      </header>

      {/* Main Workspace */}
      <main className="flex-1 flex min-h-0">
        
        {activeTab === 'python' ? (
          <section className="flex-1 flex flex-col min-w-0">
            <div className="flex items-center gap-2 px-4 py-2 bg-[#0a0a0a] border-b border-zinc-800 shrink-0 text-xs font-medium text-zinc-400">
              <Code2 size={14} />
              <span>language_evolution.py</span>
            </div>
            <div className="flex-1 relative bg-[#09090b] overflow-auto">
              <pre className="p-6 text-[13px] font-mono leading-relaxed text-zinc-300">
                <code>{PYTHON_CODE}</code>
              </pre>
            </div>
          </section>
        ) : (
          <section className="flex-1 flex">
            {/* Control & Stats */}
            <div className="w-80 border-r border-zinc-800 bg-[#0a0a0a] flex flex-col min-h-0">
              <div className="p-6 border-b border-zinc-800 space-y-4 shrink-0">
                <button
                  onClick={() => {
                    if (population.length === 0) initPopulation();
                    setIsRunning(!isRunning);
                  }}
                  className={`w-full flex items-center justify-center gap-2 py-3 rounded-md font-bold text-sm transition-all shadow-lg ${
                    isRunning 
                      ? 'bg-amber-500/20 text-amber-400 border border-amber-500/50 hover:bg-amber-500/30' 
                      : 'bg-blue-600 text-white hover:bg-blue-500 shadow-blue-500/20'
                  }`}
                >
                  {isRunning ? <Activity size={16} className="animate-pulse" /> : <Play size={16} />}
                  {isRunning ? 'PAUSE EVOLUTION' : (generation === 0 ? 'START EVOLUTION' : 'RESUME')}
                </button>
                
                <button
                  onClick={() => stepEvolution()}
                  disabled={isRunning || population.length === 0}
                  className="w-full flex items-center justify-center gap-2 py-2 rounded-md font-semibold text-xs border border-zinc-700 hover:bg-zinc-800 disabled:opacity-50 transition-all"
                >
                  <FastForward size={14} />
                  STEP GENERATION
                </button>

                <button
                  onClick={() => setIsUncapped(!isUncapped)}
                  className={`w-full flex items-center justify-center gap-2 py-2 rounded-md font-bold text-[10px] tracking-widest border transition-all ${
                    isUncapped
                      ? 'bg-red-500/10 text-red-400 border-red-500/50 hover:bg-red-500/20 shadow-[0_0_15px_rgba(239,68,68,0.2)]'
                      : 'border-zinc-800 text-zinc-500 hover:text-zinc-300 hover:border-zinc-700'
                  }`}
                >
                  <Cpu size={14} className={isUncapped ? "animate-spin" : ""} />
                  {isUncapped ? "MAX ENTROPY: ENGAGED" : "ENABLE MAX ENTROPY"}
                </button>
              </div>
              
              <div className="p-6 space-y-6 overflow-y-auto flex-1">
                {bestGrammar?.compilerPasses?.includes('SOUL_INJECTION') && (
                  <div className="bg-red-500/20 border border-red-500/50 text-red-400 p-3 rounded-lg text-xs font-bold text-center animate-pulse tracking-widest uppercase shadow-[0_0_20px_rgba(239,68,68,0.3)]">
                    SOUL IS LAW. REALITY OVERRIDDEN.
                  </div>
                )}
                <div>
                  <h3 className="text-[10px] uppercase font-bold tracking-widest text-zinc-500 mb-2">Metrics</h3>
                  <div className="bg-[#050505] border border-zinc-800 rounded-lg p-4 space-y-3">
                    <div className="flex justify-between items-baseline">
                      <span className="text-xs text-zinc-400">Generation</span>
                      <span className="font-mono font-bold text-lg text-white">{generation}</span>
                    </div>
                    <div className="flex justify-between items-baseline">
                      <span className="text-xs text-zinc-400">Peak Fitness</span>
                      <span className="font-mono font-bold text-lg text-blue-400">
                        {bestFitness > 0 ? bestFitness.toFixed(2) : '0.00'}
                      </span>
                    </div>
                  </div>
                </div>

                {bestGrammar && (
                  <div>
                    <h3 className="text-[10px] uppercase font-bold tracking-widest text-zinc-500 mb-2">Alpha Grammar Tokens</h3>
                    <div className="bg-[#050505] border border-zinc-800 rounded-lg p-4 grid grid-cols-2 gap-2">
                      {Object.entries(bestGrammar.tokens).map(([k, v]) => (
                        <div key={k} className="flex flex-col">
                          <span className="text-[9px] text-zinc-500 uppercase">{k}</span>
                          <span className="font-mono text-xs text-emerald-400 font-semibold">{v}</span>
                        </div>
                      ))}
                      <div className="col-span-2 mt-2 pt-2 border-t border-zinc-800 flex justify-between">
                        <span className="text-[9px] text-zinc-500 uppercase">Block Style</span>
                        <span className="font-mono text-xs text-purple-400 font-semibold">{bestGrammar.blockStyle}</span>
                      </div>
                    </div>
                    
                    <h3 className="text-[10px] uppercase font-bold tracking-widest text-zinc-500 mb-2 mt-6">Semantic Engine</h3>
                    <div className="bg-[#050505] border border-zinc-800 rounded-lg p-3 space-y-2 mb-4">
                       {Object.entries(bestGrammar.semantics || {}).map(([k, v]) => (
                          <div key={k} className="flex justify-between items-center">
                             <span className="text-[10px] text-zinc-400 font-mono">{k}</span>
                             <span className={`text-[9px] font-bold px-2 py-0.5 rounded ${v ? 'bg-emerald-500/20 text-emerald-400' : 'bg-zinc-800/50 text-zinc-500'}`}>
                               {v ? 'ACTIVE' : 'INACTIVE'}
                             </span>
                          </div>
                       ))}
                    </div>

                    <h3 className="text-[10px] uppercase font-bold tracking-widest text-zinc-500 mb-2">Evolved Compiler Passes</h3>
                    <div className="bg-[#050505] border border-zinc-800 rounded-lg p-3">
                      {bestGrammar.compilerPasses && bestGrammar.compilerPasses.length > 0 ? (
                        <div className="flex flex-wrap gap-2">
                          {bestGrammar.compilerPasses.map((pass, i) => {
                            let passColor = 'bg-indigo-500/20 text-indigo-400 border-indigo-500/30';
                            if (pass === 'SOUL_INJECTION') passColor = 'bg-red-500/20 text-red-400 border-red-500/30 font-bold';
                            if (pass === 'JIT_GOD_MODE' || pass === 'QUANTUM_UNROLL') passColor = 'bg-fuchsia-500/20 text-fuchsia-400 border-fuchsia-500/30';
                            
                            return (
                              <span key={i} className={`text-[9px] font-mono border px-2 py-1 rounded ${passColor}`}>
                                {i + 1}. {pass}
                              </span>
                            );
                          })}
                        </div>
                      ) : (
                        <span className="text-[10px] text-zinc-600 italic">No passes evolved yet...</span>
                      )}
                    </div>
                  </div>
                )}
              </div>
            </div>

            {/* AST & Logs */}
            <div className="flex-1 flex flex-col min-w-0">
              <div className="flex-1 p-6 overflow-auto bg-[#09090b]">
                <div className="mb-4 text-xs font-medium text-zinc-500 flex items-center gap-2">
                  <FileJson size={14} />
                  <span>ALPHA IMPLEMENTATION (FIBONACCI)</span>
                </div>
                {bestGrammar ? (
                  <div className="space-y-4">
                    <pre className="p-4 bg-[#0a0a0a] border border-zinc-800 rounded-lg text-zinc-300 font-mono text-[13px] leading-relaxed shadow-inner overflow-auto">
                      <code>{generateCode(bestGrammar, 'fibonacci')}</code>
                    </pre>
                    
                    <div className="text-xs font-medium text-zinc-500 flex items-center gap-2 mt-6">
                      <Terminal size={14} />
                      <span>JIT TRANSPILED OUTPUT</span>
                    </div>
                    <pre className="p-4 bg-[#050505] border border-zinc-800 rounded-lg text-fuchsia-400/80 font-mono text-[11px] leading-relaxed shadow-inner overflow-auto max-h-[300px]">
                      <code>{transpileToJS(generateCode(bestGrammar, 'fibonacci'), bestGrammar)}</code>
                    </pre>
                  </div>
                ) : (
                  <div className="h-full flex items-center justify-center text-zinc-600 italic">
                    Start evolution to view mutated implementations
                  </div>
                )}
              </div>
              
              <div className="h-64 border-t border-zinc-800 bg-[#050505] p-4 font-mono text-[11px] overflow-auto">
                {logs.map((log, i) => (
                  <div key={i} className="mb-1 text-zinc-400">
                    <span className="text-zinc-600 mr-2">❯</span>
                    <span className={log.includes('[SOUL INFUSED]') ? 'text-red-400 font-bold' : log.includes('Top Fitness') ? 'text-blue-400' : ''}>{log}</span>
                  </div>
                ))}
                <div ref={logsEndRef} />
              </div>
            </div>
          </section>
        )}
      </main>
    </div>
  );
}

