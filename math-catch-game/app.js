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

    // Handle keyboard events
    document.addEventListener('keydown', (e) => {
        if (game && !game.is_game_over()) {
            game.handle_key_down(e.key);
            
            // Prevent default for arrow keys and space
            if (['ArrowUp', 'ArrowDown', 'ArrowLeft', 'ArrowRight', ' '].includes(e.key)) {
                e.preventDefault();
            }
        }
    });

    document.addEventListener('keyup', (e) => {
        if (game && !game.is_game_over()) {
            game.handle_key_up(e.key);
        }
    });

    // Touch controls for mobile
    const touchControls = {
        left: false,
        right: false,
        up: false,
        down: false
    };

    // Virtual D-pad for touch devices (will be added to HTML)
    const createTouchControls = () => {
        const controlsDiv = document.createElement('div');
        controlsDiv.id = 'touch-controls';
        controlsDiv.style.cssText = `
            position: fixed;
            bottom: 20px;
            left: 20px;
            display: grid;
            grid-template-columns: repeat(3, 60px);
            grid-template-rows: repeat(3, 60px);
            gap: 5px;
            z-index: 1000;
        `;

        const buttons = [
            { pos: '1/2/2/3', dir: 'up', label: '↑' },
            { pos: '2/1/3/2', dir: 'left', label: '←' },
            { pos: '2/2/3/3', dir: 'down', label: '↓' },
            { pos: '2/3/3/4', dir: 'right', label: '→' }
        ];

        buttons.forEach(btn => {
            const button = document.createElement('button');
            button.textContent = btn.label;
            button.style.cssText = `
                grid-area: ${btn.pos};
                font-size: 24px;
                background: rgba(255, 255, 255, 0.8);
                border: 2px solid #667eea;
                border-radius: 10px;
                cursor: pointer;
                user-select: none;
            `;

            button.addEventListener('touchstart', (e) => {
                e.preventDefault();
                if (game && !game.is_game_over()) {
                    game.handle_key_down(btn.dir === 'up' ? 'w' : 
                                       btn.dir === 'down' ? 's' :
                                       btn.dir === 'left' ? 'a' : 'd');
                }
            });

            button.addEventListener('touchend', (e) => {
                e.preventDefault();
                if (game && !game.is_game_over()) {
                    game.handle_key_up(btn.dir === 'up' ? 'w' : 
                                      btn.dir === 'down' ? 's' :
                                      btn.dir === 'left' ? 'a' : 'd');
                }
            });

            controlsDiv.appendChild(button);
        });

        // Rotate button
        const rotateBtn = document.createElement('button');
        rotateBtn.textContent = '🔄';
        rotateBtn.style.cssText = `
            position: fixed;
            bottom: 20px;
            right: 20px;
            width: 80px;
            height: 80px;
            font-size: 32px;
            background: rgba(255, 221, 0, 0.9);
            border: 3px solid #cc9900;
            border-radius: 50%;
            cursor: pointer;
            user-select: none;
            z-index: 1000;
        `;

        rotateBtn.addEventListener('touchstart', (e) => {
            e.preventDefault();
            if (game && !game.is_game_over()) {
                game.handle_key_down(' ');
            }
        });

        document.body.appendChild(controlsDiv);
        document.body.appendChild(rotateBtn);

        // Show controls only on touch devices
        if ('ontouchstart' in window) {
            controlsDiv.style.display = 'grid';
            rotateBtn.style.display = 'block';
        } else {
            controlsDiv.style.display = 'none';
            rotateBtn.style.display = 'none';
        }
    };

    createTouchControls();

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
            const hpElement = document.getElementById('hp');
            if (hpElement) {
                hpElement.textContent = game.get_hp();
            }

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
