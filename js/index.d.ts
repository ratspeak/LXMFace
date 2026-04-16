/**
 * Generate a deterministic identicon SVG for an LXMF/Reticulum address.
 *
 * @param seed - Hash string (typically an LXMF hex address)
 * @param size - SVG width/height in pixels
 * @returns Self-contained SVG string
 */
export declare function lxmface(seed: string, size: number): string;
export default lxmface;
