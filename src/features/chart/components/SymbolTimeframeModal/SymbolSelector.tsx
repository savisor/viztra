import { useState, useEffect } from "react";
import { useSymbols } from "../../hooks";

interface SymbolSelectorProps {
  selectedSymbol: string;
  onSymbolSelect: (symbol: string) => void;
}

/**
 * Symbol selector component - plain JSX, no styles
 */
export function SymbolSelector({
  selectedSymbol,
  onSymbolSelect,
}: SymbolSelectorProps) {
  const { symbols, isLoading, error, hasFetched, fetchSymbols } = useSymbols();
  const [searchValue, setSearchValue] = useState("");

  useEffect(() => {
    if (!hasFetched) {
      fetchSymbols();
    }
  }, [hasFetched, fetchSymbols]);

  const filteredSymbols = symbols.filter((sym) =>
    sym.toLowerCase().includes(searchValue.toLowerCase())
  );

  return (
    <div>
      <h3>Symbol</h3>
      <div>
        <input
          type="text"
          placeholder="Search symbols..."
          value={searchValue}
          onChange={(e) => setSearchValue(e.target.value)}
        />
        <div>
          {isLoading ? (
            <div>Loading symbols...</div>
          ) : error ? (
            <div>Error: {error}</div>
          ) : filteredSymbols.length === 0 ? (
            <div>No symbols found.</div>
          ) : (
            <div>
              {filteredSymbols.map((sym) => (
                <div
                  key={sym}
                  onClick={() => onSymbolSelect(sym)}
                  style={{ cursor: "pointer" }}
                >
                  {selectedSymbol === sym && "✓ "}
                  {sym}
                </div>
              ))}
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
