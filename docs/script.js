/**
 * Suntheme Website - Interactive Effects
 * Follows the sun: dynamic background that shifts with scroll & time
 */

document.addEventListener('DOMContentLoaded', () => {
    initializeStars();
    initializeScrollEffects();
    initializeCopyButtons();
    initializeSmoothScroll();
    initializeTimeBasedTheme();
    initializeSunPosition();
});

/**
 * Create random stars in the background
 */
function initializeStars() {
    const starsContainer = document.getElementById('stars');
    const starCount = 50;

    for (let i = 0; i < starCount; i++) {
        const star = document.createElement('div');
        star.className = 'star';
        star.style.left = `${Math.random() * 100}%`;
        star.style.top = `${Math.random() * 60}%`;
        star.style.animationDelay = `${Math.random() * 2}s`;
        star.style.width = `${Math.random() * 2 + 1}px`;
        star.style.height = star.style.width;
        starsContainer.appendChild(star);
    }
}

/**
 * Scroll-based animations and effects
 */
function initializeScrollEffects() {
    const sky = document.getElementById('sky');
    const sun = document.getElementById('sun');
    const moon = document.getElementById('moon');
    const stars = document.getElementById('stars');
    const navbar = document.querySelector('.navbar');

    // Throttle scroll events for performance
    let ticking = false;

    window.addEventListener('scroll', () => {
        if (!ticking) {
            window.requestAnimationFrame(() => {
                updateScrollEffects();
                ticking = false;
            });
            ticking = true;
        }
    });

    function updateScrollEffects() {
        const scrollY = window.scrollY;
        const windowHeight = window.innerHeight;
        const docHeight = document.documentElement.scrollHeight;
        const scrollProgress = scrollY / (docHeight - windowHeight);

        // Sun position follows scroll (rises then sets)
        const sunProgress = Math.min(scrollProgress * 2, 1);
        const sunArc = Math.sin(sunProgress * Math.PI);
        const sunX = 15 + (sunProgress * 70); // Move from right to left
        const sunY = 15 + (1 - sunArc) * 40; // Arc up then down

        sun.style.right = `${sunX}%`;
        sun.style.top = `${sunY}%`;
        sun.style.opacity = sunProgress > 0.9 ? 1 - ((sunProgress - 0.9) * 10) : 0.8;

        // Moon appears as sun sets
        if (scrollProgress > 0.6) {
            const moonProgress = (scrollProgress - 0.6) / 0.4;
            moon.style.opacity = moonProgress * 0.9;
            moon.style.left = `${10 + moonProgress * 10}%`;
            moon.style.top = `${30 - moonProgress * 10}%`;
        } else {
            moon.style.opacity = 0;
        }

        // Stars fade in toward bottom
        if (scrollProgress > 0.5) {
            const starsOpacity = (scrollProgress - 0.5) / 0.5;
            stars.style.opacity = starsOpacity;
        } else {
            stars.style.opacity = 0;
        }

        // Sky gradient transitions from day to night
        updateSkyGradient(scrollProgress);

        // Navbar background intensity
        if (scrollY > 50) {
            navbar.style.background = 'rgba(255, 251, 245, 0.95)';
        } else {
            navbar.style.background = 'rgba(255, 251, 245, 0.8)';
        }
    }

    function updateSkyGradient(progress) {
        // Create smooth transition from sunrise to sunset to night
        const colors = {
            dawn: ['#FFF8E1', '#FFD4B8', '#F8BBD9', '#E1BEE7'],
            day: ['#E3F2FD', '#B3E5FC', '#81D4FA', '#4FC3F7'],
            sunset: ['#FFE0B2', '#FFAB91', '#F8BBD9', '#CE93D8'],
            night: ['#1a1a2e', '#16213e', '#0f3460', '#1a1a2e']
        };

        let gradient;
        if (progress < 0.3) {
            // Dawn to day
            gradient = interpolateGradient(colors.dawn, colors.day, progress / 0.3);
        } else if (progress < 0.6) {
            // Day to sunset
            gradient = interpolateGradient(colors.day, colors.sunset, (progress - 0.3) / 0.3);
        } else {
            // Sunset to night
            gradient = interpolateGradient(colors.sunset, colors.night, (progress - 0.6) / 0.4);
        }

        sky.style.background = `linear-gradient(180deg, ${gradient[0]} 0%, ${gradient[1]} 30%, ${gradient[2]} 60%, ${gradient[3]} 100%)`;
    }

    function interpolateGradient(colorsA, colorsB, progress) {
        return colorsA.map((colorA, i) => {
            return interpolateColor(colorA, colorsB[i], progress);
        });
    }

    function interpolateColor(colorA, colorB, progress) {
        const a = hexToRgb(colorA);
        const b = hexToRgb(colorB);
        const r = Math.round(a.r + (b.r - a.r) * progress);
        const g = Math.round(a.g + (b.g - a.g) * progress);
        const bl = Math.round(a.b + (b.b - a.b) * progress);
        return `rgb(${r}, ${g}, ${bl})`;
    }

    function hexToRgb(hex) {
        const result = /^#?([a-f\d]{2})([a-f\d]{2})([a-f\d]{2})$/i.exec(hex);
        return result ? {
            r: parseInt(result[1], 16),
            g: parseInt(result[2], 16),
            b: parseInt(result[3], 16)
        } : null;
    }

    // Intersection Observer for fade-in animations
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.classList.add('visible');
            }
        });
    }, observerOptions);

    // Observe all cards and timeline items
    document.querySelectorAll('.thesis-card, .feature-card, .timeline-item, .install-card').forEach(el => {
        el.style.opacity = '0';
        el.style.transform = 'translateY(30px)';
        el.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
        observer.observe(el);
    });

    // Add visible styles
    const style = document.createElement('style');
    style.textContent = `
        .visible {
            opacity: 1 !important;
            transform: translateY(0) !important;
        }
    `;
    document.head.appendChild(style);
}

/**
 * Copy to clipboard functionality
 */
function initializeCopyButtons() {
    const copyButtons = document.querySelectorAll('.copy-btn');

    copyButtons.forEach(btn => {
        btn.addEventListener('click', async () => {
            const textToCopy = btn.dataset.copy;

            try {
                await navigator.clipboard.writeText(textToCopy);

                // Visual feedback
                btn.classList.add('copied');
                btn.innerHTML = `
                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <path d="M20 6L9 17l-5-5"/>
                    </svg>
                `;

                setTimeout(() => {
                    btn.classList.remove('copied');
                    btn.innerHTML = `
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <rect x="9" y="9" width="13" height="13" rx="2"/>
                            <path d="M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1"/>
                        </svg>
                    `;
                }, 2000);
            } catch (err) {
                console.error('Failed to copy:', err);
            }
        });
    });
}

/**
 * Smooth scroll for anchor links
 */
function initializeSmoothScroll() {
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                const navHeight = document.querySelector('.navbar').offsetHeight;
                const targetPosition = target.offsetTop - navHeight - 20;

                window.scrollTo({
                    top: targetPosition,
                    behavior: 'smooth'
                });
            }
        });
    });
}

/**
 * Set initial theme based on current time
 */
function initializeTimeBasedTheme() {
    const hour = new Date().getHours();

    // If it's nighttime (after 7pm or before 6am), add a subtle dark overlay
    if (hour >= 19 || hour < 6) {
        document.body.classList.add('night-hint');

        // Add a subtle style for night visitors
        const style = document.createElement('style');
        style.textContent = `
            .night-hint .sky-gradient {
                filter: brightness(0.95);
            }
        `;
        document.head.appendChild(style);
    }
}

/**
 * Animate sun based on current time of day
 */
function initializeSunPosition() {
    const sun = document.getElementById('sun');
    const hour = new Date().getHours();

    // Calculate sun intensity based on time
    // Peak at noon (12), dim at dawn/dusk (6 and 18)
    let intensity = 0.8;
    if (hour >= 6 && hour <= 12) {
        intensity = 0.6 + ((hour - 6) / 6) * 0.4;
    } else if (hour > 12 && hour <= 18) {
        intensity = 1 - ((hour - 12) / 6) * 0.4;
    } else {
        intensity = 0.3;
    }

    sun.style.opacity = intensity;
}

/**
 * Typing effect for terminal (optional enhancement)
 */
function typeTerminal() {
    const terminalBody = document.querySelector('.terminal-body code');
    const originalContent = terminalBody.innerHTML;

    // This could be enhanced to type out the terminal content
    // For now, we'll keep the static content for simplicity
}

/**
 * Parallax effect for floating elements
 */
window.addEventListener('mousemove', (e) => {
    const sun = document.getElementById('sun');
    const moon = document.getElementById('moon');

    const mouseX = e.clientX / window.innerWidth - 0.5;
    const mouseY = e.clientY / window.innerHeight - 0.5;

    // Subtle parallax on sun and moon
    sun.style.transform = `translate(${mouseX * 20}px, ${mouseY * 20}px)`;
    moon.style.transform = `translate(${mouseX * -15}px, ${mouseY * -15}px)`;
});

/**
 * Add keyboard navigation support
 */
document.addEventListener('keydown', (e) => {
    // Escape key closes any modals (future enhancement)
    if (e.key === 'Escape') {
        // Future modal close logic
    }

    // Slash key focuses on install section (vim-like navigation)
    if (e.key === '/' && !e.ctrlKey && !e.metaKey) {
        const activeElement = document.activeElement;
        if (activeElement.tagName !== 'INPUT' && activeElement.tagName !== 'TEXTAREA') {
            e.preventDefault();
            document.getElementById('install').scrollIntoView({ behavior: 'smooth' });
        }
    }
});

/**
 * Performance: Reduce animations when user prefers reduced motion
 */
if (window.matchMedia('(prefers-reduced-motion: reduce)').matches) {
    document.documentElement.style.setProperty('--transition-fast', '0ms');
    document.documentElement.style.setProperty('--transition-base', '0ms');
    document.documentElement.style.setProperty('--transition-slow', '0ms');
}

/**
 * Console easter egg
 */
console.log(`
%c☀ suntheme
%cAutomatic theme switching based on sunrise and sunset.

Check it out: https://github.com/lukalavric/sun-theme
`,
'font-size: 24px; color: #FF7043; font-weight: bold;',
'font-size: 14px; color: #888;'
);
