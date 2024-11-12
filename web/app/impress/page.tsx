import { Container } from "@mantine/core";

const ImpressPage = () => {
  return (
    <Container>
      <h1>Impressum</h1>

      <p>
        Mathis Burger
        <br />
        Friedrich-Ebert-Stra&szlig;e 37
        <br />
        85055 Ingolstadt
      </p>

      <h2>Kontakt</h2>
      <p>E-Mail: kontakt@mathis-burger.de</p>

      <h2>Haftung für Links</h2>
      <p>
        Unser Angebot enthält Links zu externen Webseiten Dritter, auf deren
        Inhalte wir keinen Einfluss haben. Deshalb können wir für diese fremden
        Inhalte auch keine Gewähr übernehmen. Für die Inhalte der verlinkten
        Seiten ist stets der jeweilige Anbieter oder Betreiber der Seiten
        verantwortlich. Die verlinkten Seiten wurden zum Zeitpunkt der
        Verlinkung auf mögliche Rechtsverstöße überprüft. Rechtswidrige Inhalte
        waren zum Zeitpunkt der Verlinkung nicht erkennbar. Eine permanente
        inhaltliche Kontrolle der verlinkten Seiten ist jedoch ohne konkrete
        Anhaltspunkte einer Rechtsverletzung nicht zumutbar. Bei Bekanntwerden
        von Rechtsverletzungen werden wir derartige Links umgehend entfernen.
      </p>
      <p>
        Quelle: <a href="https://www.e-recht24.de">eRecht24</a>
      </p>
    </Container>
  );
};

export default ImpressPage;
