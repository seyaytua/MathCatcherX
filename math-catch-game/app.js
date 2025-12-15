import init, { GameWrapper } from './pkg/math_catch_game.js';

let game = null;
let animationId = null;

async function run() {
    // Initialize WASM
    await init();
    
    const canvas = document.getElementById('gameCanvas');
    const startButton = document.getElementById('startButton');
    const restartButton = document.getElementById('restartButton');
    const scoreElement = document.getElementById('score');
    const timeElement = document.getElementById('time');

    // Initialize game
    game = new GameWrapper('gameCanvas');

    // Handle canvas clicks
    canvas.addEventListener('click', (e) => {
        if (game && !game.is_game_over()) {
            const rect = canvas.getBoundingClientRect();
            const scaleX = canvas.width / rect.width;
            const scaleY = canvas.height / rect.height;
            const x = (e.clientX - rect.left) * scaleX;
            const y = (e.clientY - rect.top) * scaleY;
            game.handle_click(x, y);
        }
    });

    // Handle touch events for mobile
    canvas.addEventListener('touchstart', (e) => {
        e.preventDefault();
        if (game && !game.is_game_over()) {
            const rect = canvas.getBoundingClientRect();
            const scaleX = canvas.width / rect.width;
            const scaleY = canvas.height / rect.height;
            const touch = e.touches[0];
            const x = (touch.clientX - rect.left) * scaleX;
            const y = (touch.clientY - rect.top) * scaleY;
            game.handle_click(x, y);
        }
    });

    // Start button
    startButton.addEventListener('click', () => {
        startGame();
    });

    // Restart button
    restartButton.addEventListener('click', () => {
        restartGame();
    });

    function startGame() {
        if (game) {
            game.start();
            startButton.style.display = 'none';
            restartButton.style.display = 'none';
            gameLoop();
        }
    }

    function restartGame() {
        // Create new game instance
        game = new GameWrapper('gameCanvas');
        game.start();
        startButton.style.display = 'none';
        restartButton.style.display = 'none';
        gameLoop();
    }

    function gameLoop() {
        if (game) {
            // Update and render the game
            game.update_and_render();

            // Update UI
            scoreElement.textContent = game.get_score();
            timeElement.textContent = game.get_time_remaining();

            if (game.is_game_over()) {
                // Show restart button
                restartButton.style.display = 'inline-block';
                if (animationId) {
                    cancelAnimationFrame(animationId);
                    animationId = null;
                }
                return;
            }

            // Continue game loop
            animationId = requestAnimationFrame(gameLoop);
        }
    }

    console.log('Math Catch Game loaded successfully!');
}

run().catch(console.error);
