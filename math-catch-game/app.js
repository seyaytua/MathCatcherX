import init, { GameWrapper } from './pkg/math_catch_game.js';

let game = null;
let animationId = null;

async function run() {
    // Initialize WASM
    await init();
    
    const canvas = document.getElementById('gameCanvas');
    const startButton = document.getElementById('startButton');
    const skipButton = document.getElementById('skipButton');
    const restartButton = document.getElementById('restartButton');
    const scoreElement = document.getElementById('score');
    const timeElement = document.getElementById('time');
    const hpElement = document.getElementById('hp');

    // Modal elements
    const helpButton = document.getElementById('helpButton');
    const modeButton = document.getElementById('modeButton');
    const helpModal = document.getElementById('helpModal');
    const modeModal = document.getElementById('modeModal');
    const closeHelp = document.getElementById('closeHelp');
    const closeHelpFooter = document.getElementById('closeHelpFooter');
    const closeMode = document.getElementById('closeMode');

    // Adjust canvas size to fit screen
    function resizeCanvas() {
        const header = document.querySelector('.game-header');
        const controls = document.querySelector('.game-controls');
        const availableHeight = window.innerHeight - header.offsetHeight - controls.offsetHeight;
        const availableWidth = window.innerWidth;
        
        // Maintain aspect ratio (4:3)
        const aspectRatio = 4 / 3;
        let width = availableWidth;
        let height = width / aspectRatio;
        
        if (height > availableHeight) {
            height = availableHeight;
            width = height * aspectRatio;
        }
        
        canvas.width = Math.floor(width);
        canvas.height = Math.floor(height);
        
        // Reinitialize game if it exists
        if (game) {
            const wasPlaying = !game.is_game_over();
            game = new GameWrapper('gameCanvas');
            if (wasPlaying) {
                game.start();
                gameLoop();
            }
        }
    }

    // Initialize game
    game = new GameWrapper('gameCanvas');
    resizeCanvas();

    // Resize on window resize or orientation change
    window.addEventListener('resize', resizeCanvas);
    window.addEventListener('orientationchange', () => {
        setTimeout(resizeCanvas, 100);
    });

    // Help modal
    helpButton.addEventListener('click', () => {
        helpModal.style.display = 'flex';
    });

    closeHelp.addEventListener('click', () => {
        helpModal.style.display = 'none';
    });

    closeHelpFooter.addEventListener('click', () => {
        helpModal.style.display = 'none';
    });

    // Mode modal
    modeButton.addEventListener('click', () => {
        modeModal.style.display = 'flex';
    });

    closeMode.addEventListener('click', () => {
        modeModal.style.display = 'none';
    });

    // Close modals on backdrop click
    helpModal.addEventListener('click', (e) => {
        if (e.target === helpModal) {
            helpModal.style.display = 'none';
        }
    });

    modeModal.addEventListener('click', (e) => {
        if (e.target === modeModal) {
            modeModal.style.display = 'none';
        }
    });

    // Mode selection
    document.querySelectorAll('.mode-card').forEach(card => {
        card.addEventListener('click', () => {
            if (!card.classList.contains('mode-locked')) {
                const mode = card.dataset.mode;
                console.log('Selected mode:', mode);
                modeModal.style.display = 'none';
                // TODO: Implement mode switching
            }
        });
    });

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

    // Skip button
    skipButton.addEventListener('click', () => {
        if (game && !game.is_game_over()) {
            game.skip_problem();
        }
    });

    // Restart button
    restartButton.addEventListener('click', () => {
        restartGame();
    });

    function startGame() {
        if (game) {
            game.start();
            startButton.style.display = 'none';
            skipButton.style.display = 'inline-flex';
            restartButton.style.display = 'none';
            gameLoop();
        }
    }

    function restartGame() {
        // Create new game instance
        game = new GameWrapper('gameCanvas');
        game.start();
        startButton.style.display = 'none';
        skipButton.style.display = 'inline-flex';
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
            hpElement.textContent = game.get_hp();

            if (game.is_game_over()) {
                // Show restart button, hide skip button
                skipButton.style.display = 'none';
                restartButton.style.display = 'inline-flex';
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
