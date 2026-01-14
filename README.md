# Rusting in Rust: Cellular Automata Simulation of Corrosion Propagation

## Opis

Ovaj projekat je CA simulacija širenja rdje u 2D matrici kroz iteraciju vremena i odredjenih pravila poput vlage prostora i količine kiseonika.
Vlaga i kiseonik su vrsta pravila po kojoj se vrši širenje. Kao na primer verovatnoća prenosa korozije kao i njegov vremenski period.

Primer: svaka mala ćelija može biti "čist metal", "površinska rđa" ili "jaka rđa". Svaka ćelija gleda svoje komšije i menja se po pravilima.

## Metode i plan

Koristiću **Rust** za glavni deo simulacije i **Python** za poređenje.

Funkcionalnosti:
1. **Sekvencijalna (normalna, bez paralelizma)** verzija u Rust-u  
   - Jednostavna petlja koja ide kroz sve ćelije i menja ih po pravilima.

2. **Paralelna verzija u Rust-u** (sa threadovima)  
   - Koristiću jednostavnu biblioteku (rayon) da se više delova matrice računa u isto vreme.  

3. **Sekvencijalna i paralelna verzija u Python-u** (za poređenje)  
   - Koristiću NumPy za mrežu i multiprocessing za paralelno.

4. **Izlaz**  
   - Čuvanje stanja mreže svaku iteraciju u fajl (npr. JSON).  
   - Vizuelizacija u Rust-u (plotters biblioteka) ili Pythonu (matplotlib npr.).

5. **Dodatno** (ako stignem)  
   - Testovi brzine: koliko brže radi sa više niti.  
   - Različita pravila (promena parametra).

## Arhitektura projekta

- Glavni kod u Rust-u:  
  - Mreža: Vec<Vec<u8>> (2D tabela/matrica)  
  - Moduli: grid.rs (mreža), rules.rs (pravila), simSeq.rs (sekvencijalna simulacija), simPar.rs (paralelna simulacija), output.rs (fajlovi)
- Python:
  - NumPy za mrežu
  - multiprocessing za paralelno
- Program će imati komande: veličina mreže, broj koraka, gde da sačuva fajlove

Arhitektura projekta će se širiti i menjati kroz implementaciju, ovo je neka baza/osovna (Bazna kiselina xD)

Ovo može kasnije da se proširi za diplomski rad (npr. bolja pravila, 3D, point and click sirenje, real time vizualizacija...).
