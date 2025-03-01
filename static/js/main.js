document.addEventListener('DOMContentLoaded', () => {
    // Add the current year to the footer
    const yearElement = document.querySelector('footer .container p');
    if (yearElement) {
        yearElement.innerHTML = yearElement.innerHTML.replace('{{currentYear}}', new Date().getFullYear());
    }
    
    // Format dates
    const dateElements = document.querySelectorAll('time');
    dateElements.forEach(element => {
        const date = new Date(element.getAttribute('datetime'));
        if (!isNaN(date)) {
            element.textContent = formatDate(date);
        }
    });

    // Function to format dates
    function formatDate(date) {
        const options = { year: 'numeric', month: 'long', day: 'numeric' };
        return date.toLocaleDateString('en-US', options);
    }
});