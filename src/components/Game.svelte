<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import init, { GameState } from '../../game_logic/pkg/game_logic';

    interface Position {
        x: number;
        y: number;
        z: number;
    }

    let gameState: GameState;
    let position: Position = { x: 0, y: 0, z: 0 };
    let collectedItems: string[] = []; // Type for collected items

    // Create a set to track active movements
    const activeMovements: Set<string> = new Set();

    onMount(async () => {
        await init(); // Initialize WebAssembly
        gameState = new GameState(); // Create a new game state

        // Set up event listeners for player controls
        window.addEventListener('keydown', handleKeyDown);
        window.addEventListener('keyup', handleKeyUp);
        updatePosition(); // Initialize position
        updateCollectedItems(); // Initialize collected items
    });

    function handleKeyDown(event: KeyboardEvent) {
        activeMovements.add(event.key);
        updatePlayerMovement();
    }

    function handleKeyUp(event: KeyboardEvent) {
        activeMovements.delete(event.key);
        updatePlayerMovement();
    }

    function updatePlayerMovement() {
        const [x, y, z] = [gameState.player_position.x, gameState.player_position.y, gameState.player_position.z];

        // Reset position to current position
        let newX = x;
        let newZ = z;

        if (activeMovements.has('ArrowUp') || activeMovements.has('w')) { // Move forward
            newZ -= 1.0;
        }
        if (activeMovements.has('ArrowDown') || activeMovements.has('s')) { // Move backward
            newZ += 1.0;
        }
        if (activeMovements.has('ArrowLeft') || activeMovements.has('a')) { // Move left
            newX -= 1.0;
        }
        if (activeMovements.has('ArrowRight') || activeMovements.has('d')) { // Move right
            newX += 1.0;
        }

        gameState.move_player(newX, y, newZ); // Move to the new calculated position
        updatePosition(); // Update the displayed position
    }

    function updatePosition() {
        position = gameState.get_position(); // Use the getter to update position
    }

    function updateCollectedItems() {
        collectedItems = gameState.get_collected_items(); // Use the getter to update collected items
    }

    // Cleanup on component destruction
    onDestroy(() => {
        window.removeEventListener('keydown', handleKeyDown);
        window.removeEventListener('keyup', handleKeyUp);
    });
</script>

<div>
    <h1>Spooky Arkham Game</h1>
    <p>Use arrow keys or WASD to move and 'c' to collect an item.</p>
    <p>Player Position: (x: {position.x}, y: {position.y}, z: {position.z})</p>
    <h2>Collected Items:</h2>
    <ul>
        {#each collectedItems as item}
            <li>{item}</li>
        {/each}
    </ul>
</div>
